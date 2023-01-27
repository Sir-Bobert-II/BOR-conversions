#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Temperature
{
    Kelvin(f64),
    Celsius(f64),
    Fahrenheit(f64),
}

impl From<Temperature> for f64
{
    fn from(item: Temperature) -> Self
    {
        match item
        {
            Temperature::Kelvin(x) => x,
            Temperature::Celsius(x) => x,
            Temperature::Fahrenheit(x) => x,
        }
    }
}

impl Temperature
{
    pub fn fah(&self) -> Temperature
    {
        match *self
        {
            Self::Kelvin(x) => Self::Fahrenheit((1.8 * (x - 273.15)) + 32.0),
            Self::Celsius(x) => Self::Fahrenheit(x * 1.8 + 32.0),
            Self::Fahrenheit(x) => Self::Fahrenheit(x),
        }
    }
    pub fn cel(&self) -> Temperature
    {
        match *self
        {
            Self::Kelvin(x) => Self::Celsius(x - 273.15),
            Self::Fahrenheit(x) => Self::Celsius((x - 32.0) / 1.8),
            Self::Celsius(x) => Self::Celsius(x),
        }
    }
    pub fn kel(&self) -> Temperature
    {
        match *self
        {
            Self::Fahrenheit(x) => Self::Kelvin(((x - 32.0) / 1.8) + 273.15),
            Self::Celsius(x) => Self::Kelvin(x + 273.15),
            Self::Kelvin(x) => Self::Kelvin(x),
        }
    }
}

pub fn run(value: String, target: char) -> String
{
    let mut value = value;
    let last = value.clone().chars().last().unwrap();

    value.pop();

    let conval: Temperature = match last
    {
        'C' | 'c' => Temperature::Celsius(value.parse().unwrap_or(0.0)),

        'F' | 'f' => Temperature::Fahrenheit(value.parse().unwrap_or(0.0)),

        'K' | 'k' => Temperature::Kelvin(value.parse().unwrap_or(0.0)),

        _ => return "Error: No viable unit specified".to_string(),
    };

    let ret: f64 = match target
    {
        'F' | 'f' => f64::from(conval.fah()),
        'C' | 'c' => f64::from(conval.cel()),
        'K' | 'k' => f64::from(conval.kel()),
        _ => return "Error: No viable target specified".to_string(),
    };

    if f64::from(conval) == 0.0
    {
        return "Error: not a parseable number!".to_string();
    }

    format!("{ret:.1}{target}")
}
