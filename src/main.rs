/*
    Nap v0.1

    nap is a suspend-aware alternative to sleep for shell scripts etc.
    It ensures the program doesn't wait longer than necessary, immediately
    terminating if the system was suspended during the sleep period and the
    requested time has passed.

*/

use std::time;
use std::time::{SystemTime, UNIX_EPOCH};

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

fn main() {
    // Get sleep input from user
    let sleep: f32 = std::env::args()
        .nth(1)
        .unwrap_or_else(|| {
            eprintln!("No time given!");
            std::process::exit(1);
        })
        .parse::<f32>()
        .unwrap_or_else(|_| {
            eprintln!("unexpected time value");
            std::process::exit(1);
        });

    //convert user input to milliseconds
    let sleep_ms: u64 = (sleep * 1000.0) as u64;

    // Convert seconds to milliseconds
    let sleeptime: u64 = (sleep_ms + current_time()) / 10 * 10;

    loop {
        std::thread::sleep(time::Duration::from_millis(10));
        let c = current_time();
        if c >= sleeptime {
            break;
        }
    }
    std::process::exit(0);
}
