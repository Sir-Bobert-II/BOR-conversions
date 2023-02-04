use std::{fmt, str::FromStr};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Temperature
{
    temp: f64,
    kind: TemperatureUnit,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum TemperatureUnit
{
    Kelvin,
    Celsius,
    Fahrenheit,
}

#[derive(Error, Debug)]
pub enum ParseTempError
{
    #[error("Invalid unit provided")]
    InvalidUnit,

    #[error("Invalid number provided: {0}")]
    InvalidNumber(String),
}

impl FromStr for Temperature
{
    type Err = ParseTempError;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let mut s = s.to_lowercase();
        let kind;
        s = match s
        {
            _ if s.ends_with('c') || s.ends_with("celsius") || s.ends_with("cel") =>
            {
                kind = TemperatureUnit::Celsius;
                match s.strip_suffix('c')
                {
                    Some(x) => x.to_string(),
                    None => match s.strip_suffix("celsius")
                    {
                        Some(x) => x.to_string(),
                        None => match s.strip_suffix("cel")
                        {
                            Some(x) => x.to_string(),
                            None => s,
                        },
                    },
                }
            }

            _ if s.ends_with('f') || s.ends_with("fahrenheit") || s.ends_with("fah") =>
            {
                kind = TemperatureUnit::Fahrenheit;
                match s.strip_suffix('f')
                {
                    Some(x) => x.to_string(),
                    None => match s.strip_suffix("fahrenheit")
                    {
                        Some(x) => x.to_string(),
                        None => match s.strip_suffix("fah")
                        {
                            Some(x) => x.to_string(),
                            None => s,
                        },
                    },
                }
            }

            _ if s.ends_with('k') || s.ends_with("kelvin") =>
            {
                kind = TemperatureUnit::Kelvin;
                match s.strip_suffix('k')
                {
                    Some(x) => x.to_string(),
                    None => match s.strip_suffix("kelvin")
                    {
                        Some(x) => x.to_string(),
                        None => match s.strip_suffix("kel")
                        {
                            Some(x) => x.to_string(),
                            None => s,
                        },
                    },
                }
            }

            _ => return Err(Self::Err::InvalidUnit),
        };

        Ok(Self {
            kind,
            temp: match s.trim().parse()
            {
                Ok(x) => match kind
                {
                    TemperatureUnit::Kelvin => x,
                    TemperatureUnit::Celsius => x + 273.15,
                    TemperatureUnit::Fahrenheit => (x - 32.0) * 5.0 / 9.0 + 273.15,
                },
                Err(_) => return Err(Self::Err::InvalidNumber(s.trim().to_string())),
            },
        })
    }
}

impl std::fmt::Display for Temperature
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let (temp, unit) = match self.kind
        {
            TemperatureUnit::Kelvin => (self.temp, "Kelvin"),
            TemperatureUnit::Celsius => (self.temp - 273.15, "Celsius"),
            TemperatureUnit::Fahrenheit => ((self.temp - 273.15) * 9.0 / 5.0 + 32.0, "Fahrenheit"),
        };

        let mut m = &*format!("{temp:.3}");
        if m != "0.000"
        {
            m = m.trim_end_matches(['.', '0']);
        }
        else
        {
            m = "0";
        }

        write!(f, "{m} {unit}")
    }
}

impl Temperature
{
    pub fn as_cel(&mut self) -> &mut Self
    {
        self.kind = TemperatureUnit::Celsius;
        self
    }

    pub fn as_kel(&mut self) -> &mut Self
    {
        self.kind = TemperatureUnit::Kelvin;
        self
    }

    pub fn as_fah(&mut self) -> &mut Self
    {
        self.kind = TemperatureUnit::Fahrenheit;
        self
    }
}
