extern crate chrono;
use chrono::offset::Local;
use chrono::DateTime;
use std::time::{SystemTime, UNIX_EPOCH};
use std::cell::RefCell;

thread_local!(static SYSTEM_TIME: RefCell<SystemTime> = RefCell::new(UNIX_EPOCH));

fn set_time() {
   SYSTEM_TIME.with(|time| {
      *time.borrow_mut() = SystemTime::now();
   });
}

fn main() {
    set_time();
    let time: SystemTime = SYSTEM_TIME.with(|time| *time.borrow());
    let datetime: DateTime<Local> = time.into();
    
    println!("{}", datetime.format("%d/%m/%Y %T"));
}