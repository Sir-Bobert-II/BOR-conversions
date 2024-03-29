use super::strip_suffixes;
use chrono::{DateTime, Duration, Utc};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

#[derive(Error, Clone, Debug)]
pub enum CurrencyError
{
    #[error("NumberParseError: couldn't parse number from '{input}': {message}")]
    Parse
    {
        input: String, message: String
    },

    #[error("RequestError: couldn't request data from currency API: {message}")]
    Request
    {
        message: String
    },

    #[error("JSONParseError: couldn't parse JSON returned by currency API: {message}")]
    JsonParse
    {
        message: String
    },
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
    /// Brittish Pound
    GBP: ExchangeRateResponseDataInfo,
    /// Pakistani rupee
    PKR: ExchangeRateResponseDataInfo,
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
            "https://api.currencyapi.com/v3/latest?apikey={api_key}&currencies=EUR%2CUSD%2CCAD%2CRUB%2CJPY%2CAUD%2CAMD%2CGBP%2CPKR",
        );

        // Get the response
        if let Ok(resp) = match reqwest::blocking::get(url)
        {
            Ok(x) => x,
            Err(e) =>
            {
                return Err(CurrencyError::Request {
                    message: format!("{e}"),
                })
            }
        }
        .json::<Self>()
        {
            Ok(resp)
        }
        else
        {
            Err(CurrencyError::JsonParse {
                message: "Invalid JSON content".to_string(),
            })
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, PartialOrd)]
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

    /// Brittish Pound
    gbp: f64,

    /// Pakistani rupee
    pkr: f64,
}

impl ExchangeRates
{
    pub fn fetch(api_key: String) -> Result<Self, CurrencyError>
    {
        let resp = ExchangeRatesResponse::fetch(api_key)?;

        Ok(Self {
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
            /// Brittish Pound
            gbp: resp.data.GBP.value,
            // Pakistani rupee
            pkr: resp.data.PKR.value,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
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

    /// Brittish Pound
    Gbp,

    /// Pakistani rupee
    Pkr,
}

impl fmt::Display for CurrencyType
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let s = match self
        {
            Self::Usd => "Dollar(s) [USD]",
            Self::Eur => "Euro(s) [EUR]",
            Self::Cad => "Canadian Dollar(s) [CAD]",
            Self::Rub => "Ruble(s) [RUB]",
            Self::Jpy => "Yen [JPY]",
            Self::Aud => "Austriallian Dollar(s) [AUD]",
            Self::Amd => "Dram [AMD]",
            Self::Gbp => "Brittish Pound(s) [GBP]",
            Self::Pkr => "Pakistani rupee(s) [PKR]",
        };

        write!(f, "{s}")
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Currency
{
    converter: CurrencyConverter,
    /// The currency of the value
    currency: CurrencyType,

    /// The value of the currency stored in USD value
    value: f64,
}

impl Currency
{
    pub fn into_currency(&mut self, currency: CurrencyType) { self.currency = currency; }

    pub fn get_converter(&self) -> CurrencyConverter { self.converter.clone() }

    pub fn from_str(s: &str, converter: CurrencyConverter) -> Result<Self, CurrencyError>
    {
        let mut s = s.to_lowercase();
        let currency;
        let mut value;
        match s
        {
            _ if s.ends_with("usd") || s.ends_with("dollar") || s.starts_with('$') =>
            {
                s = strip_suffixes(s, &["usd", "dollar"]);
                s = match s.strip_prefix('$')
                {
                    Some(s) => s,
                    None => &s,
                }
                .to_string();
                currency = CurrencyType::Usd;
            }
            _ if s.ends_with("quid")
                || s.ends_with("pound")
                || s.ends_with("pounds")
                || s.ends_with("sterling")
                || s.ends_with("gbp")
                || s.starts_with('£') =>
            {
                s = strip_suffixes(s, &["quid", "pound", "pounds", "sterling", "gbp"]);
                s = match s.strip_prefix('£')
                {
                    Some(s) => s,
                    None => &s,
                }
                .to_string();
                currency = CurrencyType::Gbp;
            }
            _ if s.ends_with("eur") || s.ends_with("euro") || s.starts_with('€') =>
            {
                s = strip_suffixes(s, &["eur", "eruo"]);
                s = match s.strip_prefix('€')
                {
                    Some(s) => s,
                    None => &s,
                }
                .to_string();
                currency = CurrencyType::Eur;
            }
            _ if s.ends_with("rub") || s.ends_with("ruble") =>
            {
                s = strip_suffixes(s, &["ruble", "rub"]);
                currency = CurrencyType::Rub;
            }
            _ if s.ends_with("amd") || s.ends_with("dram") =>
            {
                s = strip_suffixes(s, &["amd", "dram"]);
                currency = CurrencyType::Amd;
            }

            _ if s.ends_with("cad") =>
            {
                s = strip_suffixes(s, &["cad"]);
                currency = CurrencyType::Cad;
            }
            _ if s.ends_with("aud") =>
            {
                s = strip_suffixes(s, &["aud"]);
                currency = CurrencyType::Aud;
            }
            _ if s.ends_with("yen") || s.ends_with("jpy") || s.starts_with('¥') =>
            {
                s = strip_suffixes(s, &["yen", "jpy"]);
                s = match s.strip_prefix('¥')
                {
                    Some(s) => s,
                    None => &s,
                }
                .to_string();
                currency = CurrencyType::Jpy;
            }
            _ if s.ends_with("pkr") || s.ends_with("pakistani rupee") =>
            {
                s = strip_suffixes(s, &["pkr", "pakistani rupee"]);
                currency = CurrencyType::Pkr;
            }
            _ =>
            {
                return Err(CurrencyError::Parse {
                    input: s,
                    message: "Invalid unit provided.".to_string(),
                })
            }
        };

        value = match s.trim().parse()
        {
            Err(e) =>
            {
                return Err(CurrencyError::Parse {
                    input: s,
                    message: format!("{e}"),
                })
            }
            Ok(v) => v,
        };

        // Store all currencies as USD
        let converter = Self::refresh_exchange_rates(converter)?;

        let exchange_rates = converter.exchange_rates;
        value /= match currency
        {
            CurrencyType::Usd => exchange_rates.usd,
            CurrencyType::Eur => exchange_rates.eur,
            CurrencyType::Cad => exchange_rates.cad,
            CurrencyType::Rub => exchange_rates.rub,
            CurrencyType::Jpy => exchange_rates.jpy,
            CurrencyType::Aud => exchange_rates.aud,
            CurrencyType::Amd => exchange_rates.amd,
            CurrencyType::Gbp => exchange_rates.gbp,
            CurrencyType::Pkr => exchange_rates.pkr,
        };

        Ok(Currency {
            value,
            currency,
            converter,
        })
    }

    /// If the exchange rates are too old, refresh them.
    fn refresh_exchange_rates(
        mut converter: CurrencyConverter,
    ) -> Result<CurrencyConverter, CurrencyError>
    {
        let now = Utc::now().time();
        let when = converter.exchange_rates.when.time();
        let max_age = converter.max_age;
        let diff = when - now;

        if diff > max_age
        {
            let key = converter.api_key.clone();
            converter.exchange_rates = ExchangeRates::fetch(key)?;
        }

        Ok(converter)
    }
}

impl fmt::Display for Currency
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        // Store all currencies as USD
        let exchange_rates = self.converter.exchange_rates;
        let value = match self.currency
        {
            CurrencyType::Usd => exchange_rates.usd,
            CurrencyType::Eur => exchange_rates.eur,
            CurrencyType::Cad => exchange_rates.cad,
            CurrencyType::Rub => exchange_rates.rub,
            CurrencyType::Jpy => exchange_rates.jpy,
            CurrencyType::Aud => exchange_rates.aud,
            CurrencyType::Amd => exchange_rates.amd,
            CurrencyType::Gbp => exchange_rates.gbp,
            CurrencyType::Pkr => exchange_rates.pkr,
        } * self.value;

        write!(f, "{value:.2} {}", self.currency)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CurrencyConverter
{
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
        Ok(Self {
            exchange_rates: ExchangeRates::fetch(api_key.clone())?,
            api_key,
            max_age,
        })
    }
}

pub fn run(
    converter: CurrencyConverter,
    input: String,
    target: String,
) -> (String, CurrencyConverter)
{
    let mut value = match Currency::from_str(&input, converter.clone())
    {
        Ok(x) => x,
        Err(e) => return (e.to_string(), converter),
    };

    let initial_value = value.to_string();

    value.into_currency(match &*target.trim().to_lowercase()
    {
        "$" | "usd" | "dollar" => CurrencyType::Usd,
        "€" | "eur" | "euro" => CurrencyType::Eur,
        "cad" => CurrencyType::Cad,
        "rub" | "ruble" => CurrencyType::Rub,
        "¥" | "yen" | "jpy" => CurrencyType::Jpy,
        "aud" => CurrencyType::Aud,
        "amd" | "dram" => CurrencyType::Amd,
        "pound" | "sterling" | "quid" => CurrencyType::Gbp,
        "pakistani rupee" | "pkr" => CurrencyType::Pkr,
        _ => return ("Error: Invalid target currency".to_string(), converter),
    });

    (format!("{initial_value} -> {value}"), value.get_converter())
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_currency_to_string_usd()
    {
        let converter = CurrencyConverter {
            exchange_rates: ExchangeRates {
                when: Utc::now(),
                eur: 0.932001,
                usd: 1.0,
                cad: 1.344352,
                rub: 71.510096,
                jpy: 132.626755,
                aud: 1.451866,
                amd: 396.62057,
                gbp: 0.831541,
                pkr: 281.850466,
            },
            api_key: "NONE".to_string(),

            max_age: Duration::hours(24),
        };

        let value = Currency::from_str("40 USD", converter).unwrap();
        assert_eq!("40.00 Dollar(s) [USD]", value.to_string())
    }

    #[test]
    fn test_currency_to_string_convert_cad()
    {
        let converter = CurrencyConverter {
            exchange_rates: ExchangeRates {
                when: Utc::now(),
                eur: 0.932001,
                usd: 1.0,
                cad: 1.344352,
                rub: 71.510096,
                jpy: 132.626755,
                aud: 1.451866,
                amd: 396.62057,
                gbp: 0.831541,
                pkr: 281.850466,
            },
            api_key: "NONE".to_string(),

            max_age: Duration::hours(24),
        };

        let mut value = Currency::from_str("40 USD", converter).unwrap();
        value.into_currency(CurrencyType::Cad);
        assert_eq!("53.77 Canadian Dollar(s) [CAD]", value.to_string())
    }

    #[test]
    fn test_currency_to_string_convert_eur()
    {
        let converter = CurrencyConverter {
            exchange_rates: ExchangeRates {
                when: Utc::now(),
                eur: 0.932001,
                usd: 1.0,
                cad: 1.344352,
                rub: 71.510096,
                jpy: 132.626755,
                aud: 1.451866,
                amd: 396.62057,
                gbp: 0.831541,
                pkr: 281.850466,
            },
            api_key: "NONE".to_string(),

            max_age: Duration::hours(24),
        };

        let mut value = Currency::from_str("80 USD", converter).unwrap();
        value.into_currency(CurrencyType::Eur);
        assert_eq!("74.56 Euro(s) [EUR]", value.to_string())
    }

    #[test]
    fn test_currency_to_string_convert_rub()
    {
        let converter = CurrencyConverter {
            exchange_rates: ExchangeRates {
                when: Utc::now(),
                eur: 0.932001,
                usd: 1.0,
                cad: 1.344352,
                rub: 71.510096,
                jpy: 132.626755,
                aud: 1.451866,
                amd: 396.62057,
                gbp: 0.831541,
                pkr: 281.850466,
            },
            api_key: "NONE".to_string(),

            max_age: Duration::hours(24),
        };

        let mut value = Currency::from_str("45.9 USD", converter).unwrap();
        value.into_currency(CurrencyType::Rub);
        assert_eq!("3282.31 Ruble(s) [RUB]", value.to_string())
    }

    #[test]
    fn test_currency_to_string_convert_jpy()
    {
        let converter = CurrencyConverter {
            exchange_rates: ExchangeRates {
                when: Utc::now(),
                eur: 0.932001,
                usd: 1.0,
                cad: 1.344352,
                rub: 71.510096,
                jpy: 132.626755,
                aud: 1.451866,
                amd: 396.62057,
                gbp: 0.831541,
                pkr: 281.850466,
            },
            api_key: "NONE".to_string(),

            max_age: Duration::hours(24),
        };

        let mut value = Currency::from_str("45.9 USD", converter).unwrap();
        value.into_currency(CurrencyType::Jpy);
        assert_eq!("6087.57 Yen [JPY]", value.to_string())
    }

    #[test]
    fn test_currency_to_string_convert_aud()
    {
        let converter = CurrencyConverter {
            exchange_rates: ExchangeRates {
                when: Utc::now(),
                eur: 0.932001,
                usd: 1.0,
                cad: 1.344352,
                rub: 71.510096,
                jpy: 132.626755,
                aud: 1.451866,
                amd: 396.62057,
                gbp: 0.831541,
                pkr: 281.850466,
            },
            api_key: "NONE".to_string(),

            max_age: Duration::hours(24),
        };

        let mut value = Currency::from_str("45.9 USD", converter).unwrap();
        value.into_currency(CurrencyType::Aud);
        assert_eq!("66.64 Austriallian Dollar(s) [AUD]", value.to_string())
    }

    #[test]
    fn test_currency_to_string_convert_amd()
    {
        let converter = CurrencyConverter {
            exchange_rates: ExchangeRates {
                when: Utc::now(),
                eur: 0.932001,
                usd: 1.0,
                cad: 1.344352,
                rub: 71.510096,
                jpy: 132.626755,
                aud: 1.451866,
                amd: 396.62057,
                gbp: 0.831541,
                pkr: 281.850466,
            },
            api_key: "NONE".to_string(),

            max_age: Duration::hours(24),
        };

        let mut value = Currency::from_str("45.9 USD", converter).unwrap();
        value.into_currency(CurrencyType::Amd);
        assert_eq!("18204.88 Dram [AMD]", value.to_string())
    }

    #[test]
    fn test_run_convert_all()
    {
        let converter = CurrencyConverter {
            exchange_rates: ExchangeRates {
                when: Utc::now(),
                eur: 0.932001,
                usd: 1.0,
                cad: 1.344352,
                rub: 71.510096,
                jpy: 132.626755,
                aud: 1.451866,
                amd: 396.62057,
                gbp: 0.831541,
                pkr: 281.850466,
            },
            api_key: "NONE".to_string(),
            max_age: Duration::hours(24),
        };

        assert_eq!(
            run(converter.clone(), "$45.9".to_string(), "usd".to_string()).0,
            "45.90 Dollar(s) [USD] -> 45.90 Dollar(s) [USD]".to_string()
        );
        assert_eq!(
            run(converter.clone(), "$45.9".to_string(), "dram".to_string()).0,
            "45.90 Dollar(s) [USD] -> 18204.88 Dram [AMD]".to_string()
        );
        assert_eq!(
            run(
                converter.clone(),
                "66.64 AUD".to_string(),
                "usd".to_string()
            )
            .0,
            "66.64 Austriallian Dollar(s) [AUD] -> 45.90 Dollar(s) [USD]".to_string()
        );
        assert_eq!(
            run(
                converter.clone(),
                "45.90 USD".to_string(),
                "aud".to_string()
            )
            .0,
            "45.90 Dollar(s) [USD] -> 66.64 Austriallian Dollar(s) [AUD]".to_string()
        );

        assert_eq!(
            run(converter.clone(), "$45".to_string(), "pkr".to_string()).0,
            "45.00 Dollar(s) [USD] -> 12683.27 Pakistani rupee(s) [PKR]".to_string()
        )
    }
}
