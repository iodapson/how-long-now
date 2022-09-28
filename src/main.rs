use chrono::prelude::{DateTime, Local};
use std::io;

fn deconstruct_date(date: &str) -> Vec<&str> {
    let date_constituents: Vec<&str> = date.split('-').collect();
    date_constituents
}

fn deconstruct_day<'a>(value: &'a str) -> &'a str {
    let vec: Vec<&str> = value.split(' ').collect();
    vec[0]
}

fn deconstruct_time<'a>(value: &'a str) -> &'a str {
    let vec: Vec<&str> = value.split(' ').collect();
    vec[1]
}

fn deconstruct_timezone<'a>(value: &'a str) -> &'a str {
    let vec: Vec<&str> = value.split(' ').collect();
    vec[2]
}

fn deconstruct_hour<'a>(value: &'a str) -> &'a str {
    let vec: Vec<&str> = value.split(':').collect();
    vec[0]
}

fn deconstruct_minute<'a>(value: &'a str) -> &'a str {
    let vec: Vec<&str> = value.split(':').collect();
    vec[1]
}

fn return_year<'a>(date: &'a Vec<&str>) -> &'a str {
    date[0]
}

fn return_month<'a>(date: &'a Vec<&str>) -> &'a str {
    date[1]
}

fn return_day<'a>(date: &'a Vec<&str>) -> &'a str {
    deconstruct_day(date[2])
}

fn return_time<'a>(date: &'a Vec<&str>) -> &'a str {
    deconstruct_time(date[2])
}

fn return_timezone<'a>(date: &'a Vec<&str>) -> &'a str {
    deconstruct_timezone(date[2])
}

fn return_hour<'a>(date: &'a Vec<&str>) -> &'a str {
    deconstruct_hour( deconstruct_time(date[2]) )
}

fn return_minutes<'a>(date: &'a Vec<&str>) -> &'a str {
    deconstruct_minute( deconstruct_time(date[2]) )
}

#[allow(unused)]
fn main() {
    //Datetime format: (yyyy-mm-ddThr:min:sec)")

    let mut target_hour_string = String::new();
    let mut target_minute_string = String::new();

    println!("What hour is your event? Your value must be between 1 - 24");
    io::stdin()
        .read_line(&mut target_hour_string)
        .expect("You input the wrong hour format");
    let target_hour: Result<i32, _> = target_hour_string.parse();
    //if target_hour.unwrap() > 24 {
        //panic!("You input a value greater than 24 or less than 1");
    //}

    println!("What minute is your event? Your value must be between 1 - 60");
    io::stdin()
        .read_line(&mut target_minute_string)
        .expect("You input the wrong minute format");
    let target_minute: Result<i32, _> = target_minute_string.parse();
    //if target_minute.unwrap() > 60 {
        //panic!("You input a value greater than 60 or less than 1");
    //}

    let local_datetime_now: DateTime<Local> = Local::now();
    
    
    let local_datetime_now_string = local_datetime_now.to_string();
    //println!("local_time_now_string: {}", local_datetime_now_string); // remove later
    println!("The time now is: {:?}", local_datetime_now_string); // remove later

    let deconstructed_date = deconstruct_date(&local_datetime_now_string);

    let returned_year = return_year(&deconstructed_date);
    let returned_month = return_month(&deconstructed_date);
    let returned_day = return_day(&deconstructed_date).trim_start();
    let returned_time = return_time(&deconstructed_date);
    let returned_timezone = return_timezone(&deconstructed_date);
    let returned_hour: i32 = return_hour(&deconstructed_date).parse().unwrap();
    let returned_minute: i32 = return_minutes(&deconstructed_date).parse().unwrap();

    println!("Year: {}, Month: {}, Day: {}, Time: {}, Timezone: {}", 
        returned_year, returned_month,
        returned_day, returned_time,
        returned_timezone
    );

    // Calculate how much time is left now
    //let minutes_left = (target_minute + 60) - returned_minute;
    //let hours_left = target_hour - returned_hour;

    //println!("You now have {}hr, {}minute(s) left to your event.", hours_left, minutes_left);

}