use serenity::{builder::CreateApplicationCommand, model::prelude::command::CommandOptionType};

/// Temperature conversions
pub mod temperature;

/// Conversions between 12 and 24 hour time
pub mod time;

/// Currency conversion
pub mod currency;

fn strip_suffixes(mut input: String, suffixes: &[&str]) -> String
{
    for suffix in suffixes
    {
        input = match input.strip_suffix(suffix)
        {
            Some(input) => input,
            None => &input,
        }.to_string()
    }
    input
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand
{
    command
        .name("conversions")
        .description("Convert between values")
        .dm_permission(true)
        .create_option(|option| {
            option
                .name("temperature")
                .kind(CommandOptionType::SubCommand)
                .description("Convert from one temperature unit to another. Supports Kelvin, Fahrenheit, and Celcius.")
                .create_sub_option(|option| {
                    option
                        .name("value")
                        .description("Original value (e.g. '65F' [Fahrenheit], '18.33C' [Celsius].")
                        .kind(CommandOptionType::String)
                        .required(true)
                })
                .create_sub_option(|option| {
                    option
                        .name("target")
                        .description("The unit to target. (e.g 'F' [Fahrenheit], 'K' [kelvin]).")
                        .kind(CommandOptionType::String)
                        .required(true)
                })
        })
        .create_option(|option| {
            option
                .name("hours")
                .kind(CommandOptionType::SubCommand)
                .description("Some people don't know how to subtract by '12'.")
                .create_sub_option(|option| {
                    option
                        .name("time")
                        .description(
                            "Time in 24h time ('6:00', '14:30'), or in 12h time ('4:44am', '6:00pm')",
                        )
                        .kind(CommandOptionType::String)
                        .required(true)
                })
        })
}
