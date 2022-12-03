extern crate clap;
use clap::{App, Arg, SubCommand};

use chrono::{DateTime, FixedOffset, Local, Utc};

fn main() {
    let matches = App::new("AoC Setup")
        .version("0.1")
        .author("Andy Coles")
        .about("Sets up a Rust project directory and downloads the input for an Advent Of Code day")
        .arg(
            Arg::with_name("year")
                .short("y")
                .long("year")
                .help("Select a different year. Default is the latest game")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .help("Select a day. Default is today")
                .takes_value(true),
        )
        .get_matches();

    let local_time = Local::now();
    let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
    let us_east_timezone = FixedOffset::west_opt(5 * 3600).unwrap();
    println!("Local time now is {}", local_time);
    println!("UTC time now is {}", utc_time);
    println!(
        "Time in AoC now is {}",
        utc_time.with_timezone(&us_east_timezone)
    );
    let aoc_release_time =
        DateTime::parse_from_str("2022 Dec 01 00:00:00.000 -0500", "%Y %b %d %H:%M:%S%.3f %z")
            .unwrap();
    let diff = aoc_release_time.signed_duration_since(utc_time);
    println!("AoC release at {}", aoc_release_time);
    println!("Countdown {}:{}", diff.num_hours(), diff.num_minutes() % 60);
    let mut current_year = utc_time
        .with_timezone(&us_east_timezone)
        .format("%y")
        .to_string()
        .parse::<i8>()
        .unwrap();
    let current_month = utc_time
        .with_timezone(&us_east_timezone)
        .format("%m")
        .to_string()
        .parse::<i8>()
        .unwrap();
    let current_day = utc_time
        .with_timezone(&us_east_timezone)
        .format("%m")
        .to_string()
        .parse::<i8>()
        .unwrap();

    if current_month < 12 {
        current_year -= 1;
    }

    if current_month == 12 && current_day < 26 {
        println!("Game is active. Today is day {}", current_day);
    } else {
        println!("Game is not running, you must specify a day");
    }
    println!("{}", current_month);
}
