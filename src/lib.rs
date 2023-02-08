use lazy_static::lazy_static;
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
        }
        .to_string()
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
                .name("currency")
                .kind(CommandOptionType::SubCommand)
                .description("Convert from one currency to another.")
                .create_sub_option(|option| {
                    option
                        .name("input")
                        .description("The input currency (e.g. '$74', '80.90 CAD', '20 quid').")
                        .kind(CommandOptionType::String)
                        .required(true)
                })
                .create_sub_option(|option| {
                    option
                        .name("target")
                        .description("The currency to convert to. (e.g 'rubles', 'usd', 'yen').")
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

lazy_static! {
    pub static ref HELP: String = {
        help::HelpMessage::new()
            .name("conversions")
            .description("Convert between values")
            .add_subcommand(
                help::HelpMessage::new()
                    .name("temperature")
                    .description("Convert from one temperature unit to another. Supports Kelvin, Fahrenheit, and Celcius")
                    .add_option({
                        help::HelpMessageOption::new()
                            .name("value")
                            .kind("String")
                            .description("Original value (e.g. '65F' [Fahrenheit], '18.33C' [Celsius])")
                            .required(true)
                            .clone()
                    })
                    .add_option({
                        help::HelpMessageOption::new()
                            .name("target")
                            .kind("String")
                            .description("The unit to target. (e.g 'F' [Fahrenheit], 'K' [kelvin])")
                            .required(true)
                            .clone()
                    })
                    .clone(),
            )
            .add_subcommand(help::HelpMessage::new()
                    .name("currency")
                    .description("Convert from one currency to another.")
                    .add_option({
                        help::HelpMessageOption::new()
                            .name("input")
                            .kind("String")
                            .description("The input currency (e.g. '$74', '80.90 CAD', '20 quid')")
                            .required(true)
                            .clone()
                    })
                    .add_option({
                        help::HelpMessageOption::new()
                            .name("target")
                            .kind("String")
                            .description("The currency to convert to (e.g 'rubles', 'usd', 'yen'). Supported currencies: USD, EUR, CAD, RUB, JPY, AUD, AMD, and GBP")
                            .required(true)
                            .clone()
                    })
                    .clone()
                )
            .add_subcommand(
                help::HelpMessage::new()
                .name("hours")
                .description("Some people don't know how to subtract by '12'")
                .add_option(
                    help::HelpMessageOption::new()
                    .name("time")
                    .description("Time in 24h time ('6:00', '14:30'), or in 12h time ('4:44am', '6:00pm')")
                    .required(true)
                    .kind("String")
                    .clone()
                ).clone()
            )
            .to_string()
    };
}
