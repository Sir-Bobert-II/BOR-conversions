use std::{fmt, str::FromStr};
use thiserror::Error;
use lazy_static::lazy_static;
use std::sync::Mutex;
use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};

lazy_static! {
    static ref EXCHANGE_RATES: Mutex<ExchangeRates>  = Mutex::new({
        ExchangeRates::fetch("KOZFwJVNUqxh8e4kXaLURtK6aaXWQIgifZnFAuxQ".to_string()).unwrap()
    });
}

#[derive(Error, Clone, Debug)]
pub enum CurrencyError
{
    #[error("ParseError: couldn't parse'{input}': {message}")]
    Parse {input: String, message: String},

    #[error("RequestError: couldn't request data from currency API: {message}")]
    Request {message: String},

    #[error("JSONParseError: couldn't parse JSON returned by currency API: {message}")]
    JsonParse {message: String},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ExchangeRates
{
    /// When the exchange rates were last fetched
    updated: DateTime<Utc>,

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
            updated: Utc::now(),

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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExchangeRatesResponse
{
    meta: ExchangeRateResponseMeta,
    data: ExchangeRateResponseData,
}

impl ExchangeRatesResponse
{
    /// Makes an http reqest using the api_key and saves this JSON
    /// data to `ECHANGE_RATE_FILE`
    pub fn fetch(api_key: String) -> Result<Self, CurrencyError>
    {
        // Construct request URL
        let url = format!(
            "https://api.currencyapi.com/v3/latest?apikey={}&currencies=EUR%2CUSD%2CCAD%2CRUB%2CJPY%2CAUD%2CAMD",
            api_key
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
                message: format!("Invalid JSON content"),
            })
        }
    }
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
    currency: CurrencyType,

    /// The value of the currency stored in USD value
    value: f64,
}

impl FromStr for Currency
{
    type Err = CurrencyError;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let mut s = s.to_lowercase();

        match s {
            _ if s.ends_with("usd") || s.ends_with("dollar") || s.starts_with('$') => {
                s = match s.strip_suffix("usd")
                {
                    Some(s) => s,
                    None => match s.strip_suffix("dollar")
                    {
                        Some(s) => s,
                        None => s,
                    }
                }
                
                s = match s.strip_prefix('$')
                {
                    Some(s) => s,
                    None => s,
                }
            },
            _ if s.ends_with("eur") || s.ends_with("euro") || s.starts_with('€') => (),
            _ if s.ends_with("rub") || s.ends_with("ruble") => (),
            _ if s.ends_with("amd") || s.ends_with("dram") => (),
            _ if s.ends_with("cad") => (),
            _ if s.ends_with("aud") => (),
            _=>(),
            
        };

        Ok(Currency {
            value: 0.0,
            currency: CurrencyType::Usd,
        })
    }
}