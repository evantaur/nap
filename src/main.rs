/*
    Nap v0.4

    nap is a suspend-aware alternative to sleep for shell scripts etc.
    It ensures the program doesn't wait longer than necessary, immediately
    terminating if the system was suspended during the sleep period and the
    requested time has passed.

*/

use chrono::Local;
use std::time::Duration;
mod switches; // Import the switches module

fn current_time() -> u64 {
    //.with_timezone(&chrono::Utc);
    Local::now().timestamp_millis() as u64
}

fn process_time(number: u64, at_times: Vec<String>, midnight: bool) -> u64 {
    // Get current time
    let now = Local::now();

    let midnight_today = now.date_naive().and_hms_opt(0, 0, 0).unwrap();
    let midnight_today_ts = midnight_today
        .and_local_timezone(Local)
        .unwrap()
        .timestamp_millis() as u64;

    // Set target time
    let mut return_time = 0;

    if number != 0 {
        return_time = number + current_time();
    }

    // Check if --midnight
    if midnight {
        let next_midnight = midnight_today_ts + 86400000;
        if return_time == 0 || next_midnight < return_time {
            return_time = next_midnight;
        }
    }

    for time_str in at_times {
        let mut time = time_str.trim().to_lowercase();
        let mut is_pm = false;

        // Check if 12h time
        if time.ends_with("am") {
            time = time.strip_suffix("am").unwrap().trim().to_string();
        } else if time.ends_with("pm") {
            time = time.strip_suffix("pm").unwrap().trim().to_string();
            is_pm = true;
        }

        // Parse the time hh:mm
        let parts: Vec<&str> = time.split(':').collect();
        if parts.len() != 2 {
            eprintln!("Invalid time formnat {time_str}");
            std::process::exit(1);
        }

        // Convert hours and minutes to milliseconds
        let mut hours: u64 = parts[0].parse().unwrap_or_else(|_| {
            eprintln!("Invalid hour in time: {time_str}");
            std::process::exit(1);
        });

        let minutes: u64 = parts[1].parse().unwrap_or_else(|_| {
            eprintln!("Invalid minute in time: {time_str}");
            std::process::exit(1);
        });

        if hours > 23 || minutes >= 60 {
            eprintln!("Time out of range {time_str}");
            std::process::exit(1);
        }

        // Convert 12-hour format to 24-hours
        if is_pm && hours != 12 {
            hours += 12;
        } else if !is_pm && hours == 12 {
            hours = 0;
        }

        // Calculate target timestamp

        let time_in_milliseconds = (hours * 3600 + minutes * 60) * 1000;
        let mut target_time = midnight_today_ts + time_in_milliseconds;

        // If target is already passed, move to the next day
        if target_time <= current_time() {
            target_time += 86400000;
        }
        if return_time == 0 || target_time < return_time {
            return_time = target_time;
        }
    }
    return_time
}

fn main() {
    // Call switches.rs and process flags
    let (number, has_midnight, at_times, sleep) = switches::process_flags().unwrap_or_else(|| {
        eprintln!("Error: Missing required arguments");
        std::process::exit(1);
    });

    let target = process_time(number.unwrap_or(0), at_times.clone(), has_midnight);
    loop {
        if current_time() >= target {
            break;
        }
        let remaining_time = target - current_time();
        let interval: u64;
        if sleep {
            interval = remaining_time;
        } else if remaining_time > 100000 {
            interval = 10000;
        } else if remaining_time > 50000 {
            interval = 5000;
        } else if remaining_time >= 10000 {
            interval = 1000;
        } else if remaining_time >= 1000 {
            interval = 500;
        } else {
            interval = 10;
        }
        std::thread::sleep(Duration::from_millis(remaining_time.min(interval.into())));
    }
    std::process::exit(0);
}
