/*fn main() {
    println!("Hello, world!");
}
*/

/*
use chrono::{Local, TimeZone};
use chrono_tz::US::Central;

fn main() {
    let time = Central
        .from_local_date(&Local::now().date_naive())
        .and_hms_opt(20, 0, 0)
        .unwrap();

    let local = Central
        .from_local_datetime(&Local::now().naive_local())
        .unwrap();

    let delta = time - local;
    let hours = delta.num_hours();
    let minutes = delta.num_minutes() % 60;

    println!("{hours}+{minutes}");
}
*/

use chrono::prelude::{DateTime, Local, NaiveDate};
fn main() {
    // Request time of event from user here:
    println!("What time is your event. (yyyy-mm-ddThr:min:sec)");

    let mut target_time  = String::new();
    io::stdin()
        .read_line(&mut target_time)
        .expect("You input the wrong date format");

    let local_time_now: DateTime<Local> = Local::now();
    
    println!("The time now is: {:?}", local_time_now);
    
}

fn deconstruct_date(local_time_now: &str, target_time: &str) -> Vec<&str> {
    let date_constituents: Vec<&str> = input.split('-').collect();
    date_constituents
}

fn compare_year(local_year: &str, target_year: &str) - u32 {
    let local_year: u32 = local_year.parse();
    let target_year: u32 = target_year.parse();
    target_year - local-local_year
}

//fn compare_month()
//
//


use chrono::prelude::{DateTime, Local};
fn main() {
    // Request time of event from user here:
    println!("What time is your event. (yyyy-mm-ddThr:min:sec)");

    //let mut target_year  = String::new();
    let mut target_hour_string = String::new();
    let mut target_minute_string = String::new();

    println!("What hour is your event (Value must be between 1 - 24");
    io::stdin()
        .read_line(&mut target_hour_string)
        .expect("You input the wrong hour format");
    let target_hour:i32 = target_hour_string.parse();

    println!("What minute is your event (Value must be between 1 - 60");
    io::stdin()
        .read_line(&mut target_minute_string)
        .expect("You input the wrong minute format");
    let target_minute: i32 = target_minute_string.parse();

    let local_time_now: DateTime<Local> = Local::now();
    
    println!("The time now is: {:?}", local_time_now);
    
}



fn deconstruct_date(local_time_now: &str, target_time: &str) -> Vec<&str> {
    let date_constituents: Vec<&str> = input.split('-').collect();
    date_constituents
}

fn compare_year(local_year: &str, target_year: &str) - u32 {
    let local_year: u32 = local_year.parse();
    let target_year: u32 = target_year.parse();
    target_year - local-local_year
}

fn compare_month()