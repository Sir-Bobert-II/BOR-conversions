fn time_to_array(input: String) -> Result<[u8; 2], String>
{
    let sections: Vec<&str> = input.split(':').collect();

    // Get the first part of the string representing the hours
    let hours_str = match sections.first()
    {
        Some(x) => *x,
        None => return Err("Error: There was no hours section found.".to_string()),
    };

    // Get the second part of the string representing the minutes.
    let minutes_str = match sections.get(1)
    {
        Some(x) => *x,
        None => return Err("Error: There was no minutes section found.".to_string()),
    };

    // Parse numbers from the strings.
    let hours: u8 = match hours_str.parse()
    {
        Ok(x) => x,
        Err(x) => return Err(format!("{x}")),
    };
    let minutes: u8 = match minutes_str.parse()
    {
        Ok(x) => x,
        Err(x) => return Err(format!("{x}")),
    };

    // Return the results
    Ok([hours.clamp(0, 24), minutes.clamp(0, 59)])
}

enum Time
{
    C12(ClockTime12),
    Base(String),
}

enum ClockTime12
{
    PostMeridiem(String),
    AnteMeridiem(String),
}

impl Time
{
    // Convert string to arrays representing the absolute time
    fn as_24h(&self) -> Result<[u8; 2], String>
    {
        match self
        {
            Self::C12(x) => match x
            {
                ClockTime12::AnteMeridiem(x) => Ok(time_to_array(x.to_string())?),
                ClockTime12::PostMeridiem(x) =>
                {
                    match time_to_array(x.to_string())
                    {
                        // Add twelve hours to the pm time to get the 24 hour time
                        Ok(x) => Ok([x[0] + 12, x[1]]),
                        Err(x) => Err(x),
                    }
                }
            },
            Self::Base(x) => match time_to_array(x.to_string())
            {
                Ok(x) => Ok(x),
                Err(x) => Err(x),
            },
        }
    }
}

pub fn run(time: String) -> String
{
    let time = time.to_lowercase(); // Make all letters lowercase.
    let to_24: bool; // What the output should be;

    // Parse the format of the time.
    let clock_time = {
        if time.ends_with("am")
        {
            to_24 = true;
            Time::C12(ClockTime12::AnteMeridiem(time.replace("am", "")))
        }
        else if time.ends_with("pm")
        {
            to_24 = true;
            Time::C12(ClockTime12::PostMeridiem(time.replace("pm", "")))
        }
        else
        {
            to_24 = false;
            Time::Base(time)
        }
    };

    if to_24
    {
        let results = match clock_time.as_24h()
        {
            Ok(x) => x,
            Err(x) => return format!("Improper input: {x}"),
        };
        format!("{:02}:{:02}", results[0], results[1])
    }
    else
    {
        let mut results = match clock_time.as_24h()
        {
            Ok(x) => x,
            Err(x) => return format!("Improper input: {x}"),
        };

        let pm = if results[0] > 12
        {
            results[0] -= 12;
            true
        }
        else
        {
            false
        };

        format!("{}:{:02}{}", results[0], results[1], {
            if pm
            {
                "pm"
            }
            else
            {
                "am"
            }
        })
    }
}
