use std::{fmt, rc::Rc, cell::RefCell};
use thiserror::Error;
use chrono::{DateTime, Utc, Duration};
use serde_derive::{Serialize, Deserialize};
use super::strip_suffixes;

#[derive(Error, Clone, Debug)]
pub enum CurrencyError
{
    #[error("NumberParseError: couldn't parse number from '{input}': {message}")]
    Parse {input: String, message: String},

    #[error("RequestError: couldn't request data from currency API: {message}")]
    Request {message: String},

    #[error("JSONParseError: couldn't parse JSON returned by currency API: {message}")]
    JsonParse {message: String},
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExchangeRatesResponse
{
    meta: ExchangeRateResponseMeta,
    data: ExchangeRateResponseData,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
struct ExchangeRateResponseData
{
    // Euro
    EUR: ExchangeRateResponseDataInfo,
    /// U.S. Dollar
    USD: ExchangeRateResponseDataInfo,
    /// Canadian Dollar
    CAD: ExchangeRateResponseDataInfo,
    /// Russian Ruble
    RUB: ExchangeRateResponseDataInfo,
    /// YEN
    JPY: ExchangeRateResponseDataInfo,
    /// Austrialian Dollar
    AUD: ExchangeRateResponseDataInfo,
    /// Armenian Dram
    AMD: ExchangeRateResponseDataInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ExchangeRateResponseMeta
{
    last_updated_at: String,
}

// Echange rates are floating point numbers that represent
// value relative to USD. USD will always be 1.0
#[derive(Serialize, Deserialize, Clone, Debug)]
struct ExchangeRateResponseDataInfo
{
    code: String,
    value: f64,
}

impl ExchangeRatesResponse
{
    /// Makes an http reqest using the api_key and saves this JSON
    /// data to `ECHANGE_RATE_FILE`
    pub fn fetch(api_key: String) -> Result<Self, CurrencyError>
    {
        // Construct request URL
        let url = format!(
            "https://api.currencyapi.com/v3/latest?apikey={api_key}&currencies=EUR%2CUSD%2CCAD%2CRUB%2CJPY%2CAUD%2CAMD",
        );

        // Get the response
        if let Ok(resp) = match reqwest::blocking::get(url)
        {
            Ok(x) => x,
            Err(e) => return Err(CurrencyError::Request{message: format!("{e}")}),
        }.json::<Self>()
        {
            Ok(resp)
        }
        else
        {
            Err(CurrencyError::JsonParse{
                message: "Invalid JSON content".to_string(),
            })
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct ExchangeRates
{
    /// When the exchange rates were last fetched
    when: DateTime<Utc>,

    // Euro
    eur: f64,
    
    /// U.S. Dollar
    usd: f64,
    
    /// Canadian Dollar
    cad: f64,
    
    /// Russian Ruble
    rub: f64,
    
    /// YEN
    jpy: f64,
    
    /// Austrialian Dollar
    aud: f64,
    
    /// Armenian Dram
    amd: f64,
}

impl ExchangeRates
{
    pub fn fetch(api_key:String) -> Result<Self, CurrencyError>
    {
        let resp = ExchangeRatesResponse::fetch(api_key)?;

        Ok(Self
        {
            /// When the exchange rates were last fetched
            when: Utc::now(),

            // Euro
            eur: resp.data.EUR.value,
            /// U.S. Dollar
            usd: resp.data.USD.value,
            /// Canadian Dollar
            cad: resp.data.CAD.value,
            /// Russian Ruble
            rub: resp.data.RUB.value,
            /// YEN
            jpy: resp.data.JPY.value,
            /// Austrialian Dollar
            aud: resp.data.AUD.value,
            /// Armenian Dram
            amd: resp.data.AMD.value,
        })

    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CurrencyType
{
    // Euro
    Eur,

    /// U.S. Dollar
    Usd,
    
    /// Canadian Dollar
    Cad,

    /// Russian Ruble
    Rub,

    /// YEN
    Jpy,

    /// Austrialian Dollar
    Aud,

    /// Armenian Dram
    Amd,
}

impl fmt::Display for CurrencyType
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let s = match self
        {
            Self::Usd => "Dollar(s) [USD]"
            Self::Eur => "Euro(s) [EUR]"
            Self::Cad => "Canadian Dollar(s) [CAD]"
            Self::Rub => "Ruble(s) [RUB]"
            Self::Jpy => "Yen [JPY]"
            Self::Aud => "Austriallian Dollar(s) [AUD]"
            Self::Amd => "Dram [AMD]"
        }

        write!(f, "{s}")
    }
}

pub struct Currency
{
    converter: Rc<RefCell<CurrencyConverter>>,

    /// The currency of the value
    currency: CurrencyType,

    /// The value of the currency stored in USD value
    value: f64,
}

impl Currency
{
    
    pub fn into_currency(&mut self, currency: CurrencyType)
    {
        self.currency = currency;
    }

    pub fn from_str(s: &str, converter: Rc<RefCell<CurrencyConverter>>) -> Result<Self, CurrencyError>
    {
        let mut s = s.to_lowercase();
        let currency;
        let mut value;
        match s {
            _ if s.ends_with("usd") || s.ends_with("dollar") || s.starts_with('$') => {
                
                s = strip_suffixes(s, &["usd", "dollar"]);
                s = match s.strip_prefix('$')
                {
                    Some(s) => s,
                    None => &s,
                }.to_string();
                currency = CurrencyType::Usd;
            },
            _ if s.ends_with("eur") || s.ends_with("euro") || s.starts_with('€') =>
            {
                s = strip_suffixes(s, &["eur", "eruo"]);
                s = match s.strip_prefix('€')
                {
                    Some(s) => s,
                    None => &s,
                }.to_string();
                currency = CurrencyType::Eur;
            },
            _ if s.ends_with("rub") || s.ends_with("ruble") => {
                s = strip_suffixes(s, &["ruble", "rub"]);
                currency = CurrencyType::Rub;
            },
            _ if s.ends_with("amd") || s.ends_with("dram") => {
                s = strip_suffixes(s, &["amd", "dram"]);
                currency = CurrencyType::Amd;
            },

            _ if s.ends_with("cad") => {
                s = strip_suffixes(s, &["cad"]);
                currency = CurrencyType::Cad;
            },
            _ if s.ends_with("aud") => {
                s = strip_suffixes(s, &["aud"]);
                currency = CurrencyType::Aud;
            },
            _=> return Err(CurrencyError::Parse {input: s, message: "Invalid unit provided.".to_string()}),
        };

        value = match s.trim().parse()
        {
            Err(e) => return Err(CurrencyError::Parse {input: s, message: format!("{e}")}),
            Ok(v) => v,
        };

        // Store all currencies as USD
        Self::refresh_exchange_rates(Rc::clone(&converter))?;

        let exchange_rates = converter.borrow().exchange_rates;
        value = match currency {
            CurrencyType::Usd => value,
            CurrencyType::Eur => value / exchange_rates.eur,
            CurrencyType::Cad => value / exchange_rates.cad,
            CurrencyType::Rub => value / exchange_rates.rub,
            CurrencyType::Jpy => value / exchange_rates.jpy,
            CurrencyType::Aud => value / exchange_rates.aud,
            CurrencyType::Amd => value / exchange_rates.amd,
        };

        Ok(Currency {
            value,
            currency,
            converter,
        })
    }

    /// If the exchange rates are too old, refresh them. 
    fn refresh_exchange_rates(converter: Rc<RefCell<CurrencyConverter>>) -> Result<(), CurrencyError>
    {
        
        let now = Utc::now().time();
        let when = converter.borrow().exchange_rates.when.time();
        let max_age = converter.borrow().max_age;
        let diff = when - now;
    
        if diff > max_age
        {
            let mut converter = converter.borrow_mut();
            let key = converter.api_key.clone();
            converter.exchange_rates = ExchangeRates::fetch(key)?;
        }
        
        Ok(())
    }
}

impl fmt::Display for Currency
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        // Store all currencies as USD
        let exchange_rates = self.converter.borrow().exchange_rates;
        let value = match self.currency
        {
            CurrencyType::Usd => self.value
            CurrencyType::Eur => exchange_rates.eur * self.value
            CurrencyType::Cad => exchange_rates.cad * self.value
            CurrencyType::Rub => exchange_rates.rub * self.value
            CurrencyType::Jpy => exchange_rates.jpy * self.value
            CurrencyType::Aud => exchange_rates.aud * self.value
            CurrencyType::Amd => exchange_rates.amd * self.value
        };

        write!(f, "{value:.2} {}", self.currency)
    }
}

pub struct CurrencyConverter {

    /// The exchange rates
    exchange_rates: ExchangeRates,
    
    /// The api key for the currency API
    api_key: String,
    
    /// The maximum valid age for the `exchange_rates` before being refreshed.
    max_age: Duration,
}

impl CurrencyConverter
{
    pub fn new(api_key: String, max_age: Duration) -> Result<Self, CurrencyError>
    {
    
        Ok(Self { exchange_rates: ExchangeRates::fetch(api_key.clone())? , api_key, max_age})
        
    }
}

pub fn run(converter: Rc<RefCell<CurrencyConverter>>, input: String, target: String) -> String
{
    let mut value = match Currency::from_str(&input,Rc::clone(&converter))
    {
        Ok(x) => x,
        Err(e) => return e.to_string(),
    };

    let initial_value = value.to_string();

    match &*target.to_lowercase()
    {
        "$" | "usd" | "dollar" => value.into_currency(CurrencyType::Usd),
        "€" | "eur" | "euro" => value.into_currency(CurrencyType::Eur),
        "cad" => value.into_currency(CurrencyType::Cad),
        "rub" | "ruble" => value.into_currency(CurrencyType::Rub),
        "yen" | "jpy" => value.into_currency(CurrencyType::Jpy),
        "aud" => value.into_currency(CurrencyType::Aud),
        "amd" | "dram" => value.into_currency(CurrencyType::Amd),
        _=> return "Error: Invalid target currency".to_string(),
    }

    format!("{initial_value} -> {value}")
   
}

#[cfg(test)]
mod tests
{
    use std::rc::Rc;
    use std::cell::RefCell;
    use super::*;
    
    #[test]
    fn test_currency_to_string_usd()
    {
        let converter = Rc::new(RefCell::new(CurrencyConverter{
            exchange_rates: ExchangeRates {
                when: Utc::now(),
                eur: 0.932001,
                usd: 1.0,
                cad: 1.344352,
                rub: 71.510096,
                jpy: 132.626755,
                aud: 1.451866,
                amd: 396.62057,
            },
            api_key: "NONE".to_string(),
            
            max_age: Duration::hours(24)
        }));
        
        let value = Currency::from_str("40 USD" ,Rc::clone(&converter)).unwrap();
        assert_eq!("40.00 USD", value.to_string())
    }
    
    #[test]
    fn test_currency_to_string_convert_cad()
    {
        let converter = Rc::new(RefCell::new(CurrencyConverter{
            exchange_rates: ExchangeRates {
                when: Utc::now(),
                eur: 0.932001,
                usd: 1.0,
                cad: 1.344352,
                rub: 71.510096,
                jpy: 132.626755,
                aud: 1.451866,
                amd: 396.62057,
            },
            api_key: "NONE".to_string(),
            
            max_age: Duration::hours(24)
        }));
        
        let mut value = Currency::from_str("40 USD" ,Rc::clone(&converter)).unwrap();
        value.into_currency(CurrencyType::Cad);
        assert_eq!("53.77 CAD", value.to_string())
    }
    
    #[test]
    fn test_currency_to_string_convert_eur()
    {
        let converter = Rc::new(RefCell::new(CurrencyConverter{
            exchange_rates: ExchangeRates {
                when: Utc::now(),
                eur: 0.932001,
                usd: 1.0,
                cad: 1.344352,
                rub: 71.510096,
                jpy: 132.626755,
                aud: 1.451866,
                amd: 396.62057,
            },
            api_key: "NONE".to_string(),
            
            max_age: Duration::hours(24)
        }));
        
        let mut value = Currency::from_str("80 USD" ,Rc::clone(&converter)).unwrap();
        value.into_currency(CurrencyType::Eur);
        assert_eq!("74.56 Euro (EUR)", value.to_string())
    }
    
    #[test]
    fn test_currency_to_string_convert_rub()
    {
        let converter = Rc::new(RefCell::new(CurrencyConverter{
            exchange_rates: ExchangeRates {
                when: Utc::now(),
                eur: 0.932001,
                usd: 1.0,
                cad: 1.344352,
                rub: 71.510096,
                jpy: 132.626755,
                aud: 1.451866,
                amd: 396.62057,
            },
            api_key: "NONE".to_string(),
            
            max_age: Duration::hours(24)
        }));
        
        let mut value = Currency::from_str("45.9 USD" ,Rc::clone(&converter)).unwrap();
        value.into_currency(CurrencyType::Rub);
        assert_eq!("3282.31 Ruble (RUB)", value.to_string())
    }
    
    #[test]
    fn test_currency_to_string_convert_jpy()
    {
        let converter = Rc::new(RefCell::new(CurrencyConverter{
            exchange_rates: ExchangeRates {
                when: Utc::now(),
                eur: 0.932001,
                usd: 1.0,
                cad: 1.344352,
                rub: 71.510096,
                jpy: 132.626755,
                aud: 1.451866,
                amd: 396.62057,
            },
            api_key: "NONE".to_string(),
            
            max_age: Duration::hours(24)
        }));
        
        let mut value = Currency::from_str("45.9 USD" ,Rc::clone(&converter)).unwrap();
        value.into_currency(CurrencyType::Jpy);
        assert_eq!("6087.57 Yen (JPY)", value.to_string())
    }
    
    #[test]
    fn test_currency_to_string_convert_aud()
    {
        let converter = Rc::new(RefCell::new(CurrencyConverter{
            exchange_rates: ExchangeRates {
                when: Utc::now(),
                eur: 0.932001,
                usd: 1.0,
                cad: 1.344352,
                rub: 71.510096,
                jpy: 132.626755,
                aud: 1.451866,
                amd: 396.62057,
            },
            api_key: "NONE".to_string(),
            
            max_age: Duration::hours(24)
        }));
        
        let mut value = Currency::from_str("45.9 USD" ,Rc::clone(&converter)).unwrap();
        value.into_currency(CurrencyType::Aud);
        assert_eq!("66.64 AUD", value.to_string())
    }
    
    #[test]
    fn test_currency_to_string_convert_amd()
    {
        let converter = Rc::new(RefCell::new(CurrencyConverter{
            exchange_rates: ExchangeRates {
                when: Utc::now(),
                eur: 0.932001,
                usd: 1.0,
                cad: 1.344352,
                rub: 71.510096,
                jpy: 132.626755,
                aud: 1.451866,
                amd: 396.62057,
            },
            api_key: "NONE".to_string(),
            
            max_age: Duration::hours(24)
        }));
        
        let mut value = Currency::from_str("45.9 USD" ,Rc::clone(&converter)).unwrap();
        value.into_currency(CurrencyType::Amd);
        assert_eq!("18204.88 Dram (AMD)", value.to_string())
    }
    
    #[test]
    fn test_run_convert_all()
    {
        let converter = Rc::new(RefCell::new(CurrencyConverter{
            exchange_rates: ExchangeRates {
                when: Utc::now(),
                eur: 0.932001,
                usd: 1.0,
                cad: 1.344352,
                rub: 71.510096,
                jpy: 132.626755,
                aud: 1.451866,
                amd: 396.62057,
            },
            api_key: "NONE".to_string(),
            max_age: Duration::hours(24),
        }));
        
        assert_eq!(run(Rc::clone(&converter), "$45.9".to_string(), "usd".to_string()), "45.90 USD -> 45.90 USD".to_string());
        assert_eq!(run(Rc::clone(&converter), "$45.9".to_string(), "dram".to_string()), "45.90 USD -> 18204.88 Dram (AMD)".to_string());
        assert_eq!(run(Rc::clone(&converter), "66.64 AUD".to_string(), "usd".to_string()), "66.64 AUD -> 45.90 USD".to_string());
        assert_eq!(run(Rc::clone(&converter), "45.90 USD".to_string(), "aud".to_string()), "45.90 USD -> 66.64 AUD".to_string());
    }
}