// target/debug/ch9-clock -s rfc2822 get
// target/debug/ch9-clock -s rfc3339 get
// target/debug/ch9-clock -s timestamp get

#[cfg(windows)]
use kernel32;
#[cfg(not(windows))]
use libc;
#[cfg(windows)]
use winapi;

use chrono::{Local, DateTime, Timelike, Datelike, TimeZone};
use clap::{Arg, Command};
use std::mem::zeroed;

struct Clock;

impl Clock {
    fn get() -> DateTime<Local> {
        Local::now()
    }

    #[cfg(windows)]
    fn set<Tz: TimeZone>(t: DateTime<Tz>) -> () {
        use chrono::Weekday;
        use kernel32::SetSystemTime;
        use winapi::{SYSTEMTIME, WORD};

        let t = t.with_timezone(&Local);

        let mut systime: SYSTEMTIME = unsafe { zeroed() };

        let dow = match t.weekday() {
            Weekday::Mon => 1,
            Weekday::Tue => 2,
            Weekday::Wed => 3,
            Weekday::Thu => 4,
            Weekday::Fri => 5,
            Weekday::Sat => 6,
            Weekday::Sun => 0,
        };

        let mut ns = t.nanosecond();

        let is_leap_second = ns > 1_000_000_000;
        if is_leap_second {
            ns -= 1_000_000_000;
        }

        systime.wYear = t.year() as WORD;
        systime.wMonth = t.month() as WORD;
        systime.wDayOfWeek = dow as WORD;
        systime.wDay = t.day() as WORD;
        systime.wHour = t.hour() as WORD;
        systime.wMinute = t.minute() as WORD;
        systime.wSecond = t.second() as WORD;
        systime.wMilliseconds = (na / 1_000_000) as WORD;

        let systime_ptr = &systime as *const SYSTEMTIME;

        unsafe {
            SetSystemTime(systime_ptr);
        }
    }

    #[cfg(not(windows))]
    fn set<Tz: TimeZone>(t: DateTime<Tz>) -> () {
        use libc::{timeval, time_t, suseconds_t, settimeofday, timezone};
        let t = t.with_timezone(&Local);
        let mut u: timeval = unsafe { zeroed() };

        u.tv_sec = t.timestamp() as time_t;
        u.tv_usec = t.timestamp_subsec_micros() as suseconds_t;

        unsafe {
            let mock_tz: *const timezone = std::ptr::null();
            settimeofday(&u as *const timeval, mock_tz);
        }
    }
}

fn main() {
    let app = Command::new("ch9-clock")
        .version("0.1.2")
        .about("Get and set the time")
        .after_help("Note: UNIX timestamps are parsed as whole \
                          seconds since 1st January 1970 0:00:00 UTC. \
                          For more accuracy, use another format.")
        .arg(
            Arg::new("action")
                .value_parser(["get", "set"])
                .default_value("get"),
        )
        .arg(
            Arg::new("std")
                .short('s')
                .long("use-standard")
                .value_parser(["rfc2822", "rfc3339", "timestamp"]) 
                .default_value("rfc3339"),
        )
        .arg(
            Arg::new("datetime")
                .help(
                    "When <action> is 'set', apply <datetime>. \
                       Otherwise ignore."
                ),
        );

    let args = app.get_matches();
    let action = args.get_one::<String>("action").unwrap();
    let std = args.get_one::<String>("std").unwrap();

    if action == "set" {
        let t_ = args.get_one::<String>("datetime").unwrap();

        let parser = match std.as_str() {
            "rfc2822" => DateTime::parse_from_rfc2822,
            "rfc3339" => DateTime::parse_from_rfc3339,
            _ => unimplemented!(),

        };

        let err_msg = format!("Unable to parse {} according to {}", t_, std);
        let t = parser(t_).expect(&err_msg);

        Clock::set(t);
    }

    // Use system time zone
    let now = Clock::get();
    match std.as_str() {
        "rfc2822" => println!("{}", now.to_rfc2822()),
        "rfc3339" => println!("{}", now.to_rfc3339()),
        "timestamp" => println!("{}", now.timestamp()),
        _ => unreachable!(),
    }
}