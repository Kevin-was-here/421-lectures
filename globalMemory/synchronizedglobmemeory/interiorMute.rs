extern crate chrono;
use chrono::offset::Local;
use chrono::DateTime;
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::RwLock;

static SYSTEM_TIME: RwLock<SystemTime> = RwLock::new(UNIX_EPOCH);

fn set_time() {
   let mut time = SYSTEM_TIME.write().unwrap();
   *time = SystemTime::now();
   // the lock is released when time goes out of scope
}

fn main() {
    set_time();
    let time: SystemTime = *SYSTEM_TIME.read().unwrap();
    let datetime: DateTime<Local> = time.into();

    println!("{}", datetime.format("%d/%m/%Y %T"));
}