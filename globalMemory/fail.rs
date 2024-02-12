static mut DBG_LEVEL: u8 = 1;     // Type must be specified
static PRODUCTION: bool = false;  // Initial value must be specified

//This fails to compile because rust does not allow mutable statics to be initialized at runtime
fn main() {
   if PRODUCTION {
       DBG_LEVEL = 0;
   }
   println!("Running at debug level {}", DBG_LEVEL);
}