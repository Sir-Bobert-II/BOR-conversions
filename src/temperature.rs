pub fn run(input: &str, target: &str) -> String
{
    let mut temp = match Temperature::from_str(input)
    {
        Ok(x)=>x,
        Err(e) => return format!("{}", e.message),
    };
    let original = temp;
    match &*target.to_lowercase()
    {
        "k" | "kelvin"| "kel" => temp.to_kel(),
        "c" | "celsius" | "cel" => temp.to_cel(),
        "f" | "fahrenheit"| "fah" => temp.to_fah(),
        _=> return "A valid target couldn't be found".to_string()
    };
    
    format!("{original} -> {temp}")
}

use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Temperature
{
    temp: f64,
    kind: TemperatureUnit
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum TemperatureUnit
{
    Kelvin,
    Celsius,
    Fahrenheit,
}

#[derive(Debug)]
pub struct ParseTempError{ pub message: String}

impl FromStr for Temperature
{
    type Err = ParseTempError;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let mut s = s.to_lowercase();
        let kind;
        s = match s
        {
            _ if s.ends_with("c") || s.ends_with("celsius") || s.ends_with("cel")=> 
            {
                kind = TemperatureUnit::Celsius;
                match s.strip_suffix("c")
                {
                    Some(x) => x.to_string(),
                    None =>
                    {
                        match s.strip_suffix("celsius")
                        {
                            Some(x) => x.to_string(),
                            None => match s.strip_suffix("cel")
                            {
                                Some(x) => x.to_string(),
                                None => s,
                            },
                        }
                    },
                }
            }
            
            _ if s.ends_with("f") || s.ends_with("fahrenheit") || s.ends_with("fah")=> 
            {
                kind = TemperatureUnit::Fahrenheit;
                match s.strip_suffix("f")
                {
                    Some(x) => x.to_string(),
                    None =>
                    {
                        match s.strip_suffix("fahrenheit")
                        {
                            Some(x) => x.to_string(),
                            None => match s.strip_suffix("fah")
                            {
                                Some(x) => x.to_string(),
                                None => s,
                            },
                        }
                    },
                }
            }

            _ if s.ends_with("k") || s.ends_with("kelvin")=> 
            {
                kind = TemperatureUnit::Kelvin;
                match s.strip_suffix("k")
                {
                    Some(x) => x.to_string(),
                    None =>
                    {
                        match s.strip_suffix("kelvin")
                        {
                            Some(x) => x.to_string(),
                            None => match s.strip_suffix("kel")
                            {
                                Some(x) => x.to_string(),
                                None => s,
                            },
                        }
                    },
                }
            }

            _=> return Err(Self::Err {message: "A viable unit hasn't been found".to_string()}),
        };

        Ok(Self {
            kind,
            temp: match s.trim().parse() {
                Ok(x)=> match kind 
                {
                    TemperatureUnit::Kelvin => x,
                    TemperatureUnit::Celsius => x + 273.15,
                    TemperatureUnit::Fahrenheit => (x - 32.0) * 5.0/9.0 + 273.15,
                }
                Err(e) => return Err(Self::Err {message: e.to_string()})
            }
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
            TemperatureUnit::Fahrenheit => ((self.temp - 273.15) * 9.0/5.0 + 32.0, "Fahrenheit"),
        };
        
        let mut m = format!("{temp:.3}");
        if m != "0.000".to_string()
        {
            m = m.trim_end_matches(['.', '0']).to_string();
        }
        else
        {
            m = "0".to_string();
        }

        write!(f, "{m} {unit}")
    }
}

impl Temperature{

    pub fn to_cel(&mut self)-> &mut Self
    {
        self.kind = TemperatureUnit::Celsius;
        self
    }
    
    pub fn to_kel(&mut self)-> &mut Self
    {
        self.kind = TemperatureUnit::Kelvin;
        self
    }
    
    pub fn to_fah(&mut self)-> &mut Self
    {
        self.kind = TemperatureUnit::Fahrenheit;
        self
    }
}