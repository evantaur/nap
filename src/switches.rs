/*
    Switches.rs - Handle the logic of switches
*/

use clap::{command, value_parser, Arg, ArgAction};

pub fn parse_number_with_suffix(number: &str) -> Result<u64, String> {
    let is_seconds = number.ends_with('s');
    let is_num_or_float = number.chars().all(|c| c.is_numeric() || c == '.');
    let multiplier = if is_seconds ^ is_num_or_float {
        1.0 // seconds (default)
    } else if number.ends_with("m") {
        60.0
    } else if number.ends_with("h") {
        3600.0
    } else if number.ends_with("d") {
        86400.0
    } else {
        return Err(format!("Invalid time format: {number}"));
    };
    let numeric = number.trim_end_matches(|c: char| !c.is_numeric() && c != '.');
    let value: f64 = numeric
        .parse()
        .map_err(|_| format!("Invalid number: {}", numeric))?;

    let milliseconds = (value * multiplier * 1000.0).round() as u64;

    Ok(milliseconds)
}

pub fn process_flags() -> Option<(Option<u64>, bool, Vec<String>, bool)> {
    let matches = command!() // Requires `cargo` feature
        .arg(
            Arg::new("NUMBER")
                .help("Time to wait in seconds or with suffix")
                .long_help(
                    "Pause for <NUMBER> of seconds. SUFFIX may be:
  - 's' for seconds (default)
  - 'm' for minutes
  - 'h' for hours
  - 'd' for days
Specify the time to pause with the optional suffix. If no suffix is provided, seconds is assumed.",
                )
                .required(false),
        )
        .arg(
            Arg::new("midnight")
                .long("midnight")
                .help("Pause the timer when it reaches midnight.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("at")
                .long("at")
                .help("Set a specific time to pause the timer (e.g., 18:00 or 6:00PM). You can use either 24-hour or 12-hour format.")
                .value_parser(value_parser!(String))
                .action(ArgAction::Append)
                .num_args(1),
        )
        .arg(
            Arg::new("sleep")
                .long("sleep")
                .help("Just do what sleep does")
                .action(ArgAction::SetTrue),
        )
        
        .get_matches();

    // Check if Midnight or At is provided
    let has_at = matches.contains_id("at");
    let has_midnight = matches.get_flag("midnight");
    let sleep = matches.get_flag("sleep");

    // Check number
    let number = matches.get_one::<String>("NUMBER").cloned();

    let parsed_number = number.map(|number_str| {
        parse_number_with_suffix(&number_str).unwrap_or_else(|e| {
            eprintln!("{e}");
            std::process::exit(1);
        })
    });

    // If neighter "at" or "midnight" was provided, Require "Number"
    if parsed_number.is_none() && !has_at && !has_midnight {
        eprintln!("Error: You must provide either 'Number', '--at', or '--midnight', or any combination of them.");
        std::process::exit(1);
    }

    // If `--at` is provided, collect the times
    let at_times = matches
        .get_many::<String>("at")
        .unwrap_or_default()
        .cloned()
        .collect::<Vec<String>>();

    Some((parsed_number, has_midnight, at_times, sleep))
}
