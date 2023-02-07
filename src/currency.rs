use std::{fmt, rc::Rc, cell::RefCell};
use thiserror::Error;
use chrono::{DateTime, Utc, Duration};
use serde_derive::{Serialize, Deserialize};
use super::strip_suffixes;

pub fn run(input: String, target: String, api_key: String, max_age: Duration) -> String
{
    let converter = match CurrencyConverter::new(api_key)
    {
        Ok(x) => Rc::new(RefCell::new(x)),
        Err(e) => return e.to_string(),
    };

    let mut value = match Currency::from_str(&input,Rc::clone(&converter), max_age)
    {
        Ok(x) => x,
        Err(e) => return e.to_string(),
    };

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

    value.to_string()
}

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

    pub fn from_str(s: &str, converter: Rc<RefCell<CurrencyConverter>>, max_age: Duration) -> Result<Self, CurrencyError>
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

        value = match s.parse()
        {
            Err(e) => return Err(CurrencyError::Parse {input: s, message: format!("{e}")}),
            Ok(v) => v,
        };

        // Store all currencies as USD
        Self::refresh_exchange_rates(Rc::clone(&converter), max_age)?;

        let exchange_rates = converter.borrow().exchange_rates;
        value = match currency {
            CurrencyType::Usd => value,
            CurrencyType::Eur => exchange_rates.eur / value,
            CurrencyType::Cad => exchange_rates.cad / value,
            CurrencyType::Rub => exchange_rates.rub / value,
            CurrencyType::Jpy => exchange_rates.jpy / value,
            CurrencyType::Aud => exchange_rates.aud / value,
            CurrencyType::Amd => exchange_rates.amd / value,
        };

        Ok(Currency {
            value,
            currency,
            converter,
        })
    }

    /// If the exchange rates are too old, refresh them. 
    fn refresh_exchange_rates(converter: Rc<RefCell<CurrencyConverter>>, max_diff: Duration) -> Result<(), CurrencyError>
    {
        
        let now = Utc::now().time();
        let when = converter.borrow().exchange_rates.when.time();
        let diff = when - now;
    
        if diff > max_diff
        {
            let mut converter = converter.borrow_mut();
            let key = converter.api_key.clone();
            converter.exchange_rates = ExchangeRates::fetch(key)?;
        }
        
        Ok(())
    }
}

impl std::fmt::Display for Currency
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        // Store all currencies as USD
        let exchange_rates = self.converter.borrow().exchange_rates;
        let (value, currency) = match self.currency
        {
            CurrencyType::Usd => (self.value, "USD"),
            CurrencyType::Eur =>(exchange_rates.eur * self.value, "Euro (EUR)"),
            CurrencyType::Cad => (exchange_rates.cad * self.value, "Cad (CAD)"),
            CurrencyType::Rub => (exchange_rates.rub * self.value, "Ruble (RUB)"),
            CurrencyType::Jpy => (exchange_rates.jpy * self.value, "Yen (JPY)"),
            CurrencyType::Aud => (exchange_rates.aud * self.value, "AUD"),
            CurrencyType::Amd => (exchange_rates.amd * self.value, "Dram (Amd)"),
        };

        write!(f, "{value:.2} {currency}")
    }
}

pub struct CurrencyConverter { exchange_rates: ExchangeRates, api_key: String }

impl CurrencyConverter
{
    pub fn new(api_key: String) -> Result<Self, CurrencyError>
    {
    
        Ok(Self { exchange_rates: ExchangeRates::fetch(api_key.clone())? , api_key })
        
    }
}