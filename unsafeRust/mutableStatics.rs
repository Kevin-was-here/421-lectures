extern crate chrono;
use chrono::offset::Local;
use chrono::DateTime;
use std::time::{SystemTime, UNIX_EPOCH};

static mut SYSTEM_TIME: SystemTime = UNIX_EPOCH;

fn set_time() {
    unsafe {
        SYSTEM_TIME = SystemTime::now();
    }
}

fn main() {
    set_time();
    let datetime: DateTime<Local>;
    unsafe {
        datetime = SYSTEM_TIME.into();
    }
    println!("{}", datetime.format("%d/%m/%Y %T"));
}