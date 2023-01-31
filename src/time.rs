use std::{fmt, str::FromStr};

pub fn run(t: String) -> String
{
    if let Ok(mut time) = Time::from_str(&t)
    {
        let original = time.clone();
        let opposite = *time.to_opposite();
        format!("{original} -> {opposite}")
    }
    else
    {
        format!("'{t}' is in improper form. Examples: '12:20 PM' or '17:00:08'.")
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub enum TimeNotation
{
    TwelveHour,
    #[default]
    TwentyFourHour,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Default)]
/// The number of hours are stored in 24 hour notation.
pub struct Time
{
    kind: TimeNotation,
    hours: u8,
    minutes: u8,
    seconds: u8,
}

#[derive(Debug)]
pub struct ParseTimeError
{
    message: String,
}

impl FromStr for Time
{
    type Err = ParseTimeError;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let mut s = s.to_lowercase();
        let mut kind = TimeNotation::TwentyFourHour;
        let mut pm = false;

        if s.ends_with("pm")
        {
            s = match s.strip_suffix("pm")
            {
                Some(x) =>
                {
                    kind = TimeNotation::TwelveHour;
                    pm = true;
                    x.to_string()
                }
                None => s,
            }
        }
        else if s.ends_with("am")
        {
            s = match s.strip_suffix("am")
            {
                Some(x) =>
                {
                    kind = TimeNotation::TwelveHour;
                    x.to_string()
                }
                None => s,
            }
        }

        let sections: Vec<&str> = s.split(':').collect();
        match sections.len()
        {
            1 | 2 | 3 =>
            {
                let mut i = 0;
                let mut time = Self::new(kind);
                while i < sections.len()
                {
                    match sections[i].trim().parse()
                    {
                        Ok(x) => match i
                        {
                            0 =>
                            {
                                time.hours = {
                                    if kind == TimeNotation::TwelveHour
                                    {
                                        if !pm && x == 12
                                        {
                                            x - 12
                                        }
                                        else if pm && x < 12
                                        {
                                            x + 12
                                        }
                                        else
                                        {
                                            x
                                        }
                                    }
                                    else
                                    {
                                        x
                                    }
                                }
                            }
                            1 => time.minutes = x,
                            2 => time.seconds = x,
                            _ => (),
                        },
                        Err(message) =>
                        {
                            return Err(Self::Err {
                                message: format!("{message} {}", sections[i]),
                            })
                        }
                    }
                    i += 1;
                }
                Ok(time)
            }
            _ => Err(Self::Err {
                message: "Too many sections!".to_string(),
            }),
        }
    }
}

impl std::fmt::Display for Time
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self.kind
        {
            TimeNotation::TwelveHour =>
            {
                let (period, hours) = if self.hours == 0 || self.hours == 24
                {
                    ("AM", 12)
                }
                else if self.hours > 12
                {
                    ("PM", self.hours - 12)
                }
                else if self.hours == 12
                {
                    ("PM", self.hours)
                }
                else
                {
                    ("AM", self.hours)
                };

                write!(
                    f,
                    "{hours:02}:{:02}:{:02} {period}",
                    self.minutes, self.seconds
                )
            }
            TimeNotation::TwentyFourHour =>
            {
                write!(
                    f,
                    "{:02}:{:02}:{:02}",
                    self.hours, self.minutes, self.seconds
                )
            }
        }
    }
}

impl Time
{
    pub fn new(kind: TimeNotation) -> Self
    {
        Self {
            kind,
            ..Default::default()
        }
    }

    pub fn to_24(&mut self) -> &mut Self
    {
        self.kind = TimeNotation::TwentyFourHour;
        self
    }

    pub fn to_12(&mut self) -> &mut Self
    {
        self.kind = TimeNotation::TwelveHour;
        self
    }

    pub fn to_opposite(&mut self) -> &mut Self
    {
        match self.kind
        {
            TimeNotation::TwentyFourHour => self.to_12(),
            _ => self.to_24(),
        }
    }
}

#[cfg(test)]
pub mod test
{
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_from_str_12h_pm()
    {
        let t = Time::from_str("12:00:00 PM").unwrap();
        let time = Time {
            kind: TimeNotation::TwelveHour,
            hours: 12,
            minutes: 0,
            seconds: 0,
        };

        assert_eq!(t, time);
    }

    #[test]
    fn test_from_str_12h_am()
    {
        let test = Time::from_str("6:40:00 AM").unwrap();
        let time = Time {
            kind: TimeNotation::TwelveHour,
            hours: 6,
            minutes: 40,
            seconds: 0,
        };

        assert_eq!(test, time);
    }

    #[test]
    fn test_from_str_24h_pm()
    {
        let test = Time::from_str("14:50:11").unwrap();
        let time = Time {
            kind: TimeNotation::TwentyFourHour,
            hours: 14,
            minutes: 50,
            seconds: 11,
        };

        assert_eq!(test, time);
    }

    #[test]
    fn test_from_str_24h_am()
    {
        let test = Time::from_str("0:50:11").unwrap();
        let time = Time {
            kind: TimeNotation::TwentyFourHour,
            hours: 0,
            minutes: 50,
            seconds: 11,
        };

        assert_eq!(test, time);
    }

    #[test]
    fn test_stringify_12h_am()
    {
        let test = Time::from_str("6:40:00 AM").unwrap();
        assert_eq!(test.to_string(), "06:40:00 AM".to_string())
    }

    #[test]
    fn test_stringify_12h_pm()
    {
        let test = Time::from_str("6:45:02 PM").unwrap();
        assert_eq!(test.to_string(), "06:45:02 PM".to_string())
    }

    #[test]
    fn test_stringify_24h_pm()
    {
        let test = Time::from_str("14:40:00").unwrap();
        assert_eq!(test.to_string(), "14:40:00".to_string())
    }

    #[test]
    fn test_stringify_24h_am()
    {
        let test = Time::from_str("00:45:02").unwrap();
        dbg!(&test);
        assert_eq!(test.to_string(), "00:45:02".to_string())
    }

    #[test]
    fn test_convert_12h_to_24h_am()
    {
        let mut test = Time::from_str("6:40:00 AM").unwrap();
        test.to_24();
        let time = Time {
            kind: TimeNotation::TwentyFourHour,
            hours: 6,
            minutes: 40,
            seconds: 0,
        };

        assert_eq!(test, time);
    }

    #[test]
    fn test_convert_12h_to_24h_pm()
    {
        let mut test = Time::from_str("6:45:05 PM").unwrap();
        test.to_24();
        let time = Time {
            kind: TimeNotation::TwentyFourHour,
            hours: 18,
            minutes: 45,
            seconds: 5,
        };

        assert_eq!(test, time);
    }

    #[test]
    fn test_convert_24h_to_12h_am()
    {
        let mut test = Time::from_str("6:45:05").unwrap();
        test.to_12();
        let time = Time {
            kind: TimeNotation::TwelveHour,
            hours: 6,
            minutes: 45,
            seconds: 5,
        };

        assert_eq!(test, time);
    }

    #[test]
    fn test_convert_24h_to_12h_pm()
    {
        let mut test = Time::from_str("24:0:05").unwrap();
        test.to_12();
        let time = Time {
            kind: TimeNotation::TwelveHour,
            hours: 24,
            minutes: 0,
            seconds: 5,
        };

        assert_eq!(test, time);
    }
}
