use std::sync::atomic::{AtomicU8, Ordering};

static DBG_LEVEL: AtomicU8 = AtomicU8::new(1);
static PRODUCTION: bool = true;  // Initial value must be available at compile time

fn main() {
   if PRODUCTION {


        //ordering is relaxed because we don't care about the order of the operations
       DBG_LEVEL.store(0, Ordering::Relaxed);
   }
   println!("Running at debug level {}", DBG_LEVEL.load(Ordering::Relaxed));
}