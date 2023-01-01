use chrono::{DateTime, Local, NaiveDate, Utc};
use clap::Parser;

/// Struct Cli helps in parsing passed-in commandline arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// hour of event
    #[arg(short = 'l', long = "hour")]
    hour: u32,
    /// minute of event
    #[arg(short = 'm', long = "min", default_value_t = 0)]
    minutes: u32,
    /// day of event
    #[arg(short = 'd', long = "day")]
    day: Option<u32>,
    /// month of event
    #[arg(short = 'o', long = "month")]
    month: Option<u32>,
    /// year of event
    #[arg(short = 'y', long = "year")]
    year: Option<u32>,
}

#[allow(unused)]
fn main() {
    let current_datetime: DateTime<Utc> = Utc::now();
    let current_datetime_string = current_datetime.to_string();

    let args = Cli::parse();
    let event_hour = args.hour;
    let event_minute = args.minutes;

    let event_day: Option<u32> = if let Some(val) = args.day {
        Some(val)
    } else {
        Some("0".parse::<u32>().unwrap())
    };

    let event_month: Option<u32> = if let Some(val) = args.month {
        Some(val)
    } else {
        Some("0".parse::<u32>().unwrap())
    };

    let event_year: Option<u32> = if let Some(val) = args.year {
        Some(val)
    } else {
        Some("0".parse::<u32>().unwrap())
    };

    println!(
        "\nUtc calender date now:  {}",
        current_datetime.format("[%a, %e %b %Y]")
    );
    println!("Time:  {} UTC\n", current_datetime.format("[%T]"));

    println!("Alt time::");
    alt_time(current_datetime);
}

fn alt_time(current_date_time: DateTime<Utc>) {
    let utc = current_date_time;
    let local_datetime_now: DateTime<Local> = Local::now();
    println!(
        "utc is {:?} \n..and local is: {:?}",
        utc, local_datetime_now
    );

    let local_datetime_now_string = local_datetime_now.to_string();

    let (year, remaining) = local_datetime_now_string.split_once('-').unwrap();
    let (month, remaining) = remaining.split_once('-').unwrap();
    let (day, remaining) = remaining.split_once(' ').unwrap();
    let (hour, remaining) = remaining.split_once(':').unwrap();
    let (minutes, remaining) = remaining.split_once(':').unwrap();
    let (seconds, remaining) = remaining.split_once('.').unwrap();

    println!(
        "\nyear: {}\nmonth: {}\nday: {}
        \nhour: {}\nminutes: {}\nseconds: {}
        \nremaining: {}\n",
        year, month, day, hour, minutes, seconds, remaining,
    );

    time_diff_experiment(NaiveDate::from_ymd(
        year.parse::<i32>().unwrap(),
        month.parse::<u32>().unwrap(),
        day.parse::<u32>().unwrap(),
    ));
}

fn time_diff_experiment(naive_date: NaiveDate) {
    let duration = NaiveDate::from_ymd(2023, 9, 11)
        .and_hms_opt(2, 4, 6)
        .unwrap()
        .signed_duration_since(naive_date.and_hms_opt(8, 9, 12).unwrap());
    println!("Diff experiment: {:?}", duration);
    println!("Diff experiment in weeks: {:?}", duration.num_weeks());
    println!("Diff experiment days: {:?}", duration.num_days());
    println!("Diff experiment hours: {:?}\n", duration.num_hours());
    /*println!(
        "Diff experiment minutes: {:?}",
        duration.num_minutes().gt(&400)
    );*/
}

// OLD CODE PIECE (SOON TO BE REMOVED)
/*fn experiment() {
  //use chrono::prelude::*;

  let utc: DateTime<Utc> = Utc::now();
  let local: DateTime<Local> = Local::now();
}

fn experiment() {
    println!("BEGIN EXPERIMENT");
    let utc_dt: DateTime<Utc> = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(20, 32), Utc);
    // Result: 1970-01-01 00:00:20.000000032 UTC
    println!("This is dt: {}", utc_dt);
    //Result:  ("1970", "01-01 00:00:20.000000032 UTC")
    let split_result = utc_dt.to_string();
    //split_once("-").unwrap();
    println!("This is split_result: {:?}", split_result.split_once('-').unwrap());

    // Result: 2022-10-12 18:01:29.403662700 +01:00
    let local_datetime_now: DateTime<Local> = Local::now();
    println!("local_datetime_now: {}", local_datetime_now);

    let utc_now = Utc::now().to_string();
    println!("\nUtc::today is: {}\n", utc_now);

    // Format: 2022-10-12 17:32:00.763215800 UTC
    let (year, remainder_dt) = utc_now.split_once('-').unwrap();
    println!("The year is: {year}");
    println!("remainder_dt: {remainder_dt}");
    let (month, remainder_dt) = remainder_dt.split_once('-').unwrap();
    println!("The month is: {month}");
    println!("remainder_dt: {remainder_dt}");
    let (day, remainder_time) = remainder_dt.split_once(' ').unwrap();
    println!("The day is: {day}");
    println!("remainder_time: {remainder_time}");
    let (hour, remainder_time) = remainder_time.split_once(':').unwrap();
    println!("The hour is: {hour}");
    println!("remainder time: {remainder_time}");
    let (minute, remainder_time) = remainder_time.split_once(':').unwrap();
    println!("The minute is: {minute}");
    println!("remainder time: {remainder_time}");
    let (seconds, remainder_time) = remainder_time.split_once(':').unwrap();
    println!("The seconds is: {seconds}");
    println!("remainder time: {remainder_time}");

    println!("\nNaiveDate, NaiveTime, NaiveDateTime stuff...");
    let yr: i32 = year.try_into().unwrap();
    //let y: i32 = yr.try_into().unwrap();
    //let m: u32 = month.try_into().unwrap();
    //let dy: u32 = day.try_into().unwrap();
    //let d = NaiveDate::from_ymd(y, m, dy);
    //let t = NaiveTime::from_hms(hour, minute, seconds);
    //let dt = NaiveDateTime::new(d, t);
    //println!("NaiveDateTime: {}", dt.to_string());
    //println!("yr: {:?}", yr);
}
*/
