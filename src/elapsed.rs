use std::time::{Duration, SystemTime};
use std::thread::sleep;

fn main() {
   let now = SystemTime::now();

   // we sleep for 2 seconds
   sleep(Duration::new(2, 0));
   match now.elapsed() {
       Ok(elapsed) => {
           // it prints '2'
           println!("{}", elapsed.as_secs());
       }
       Err(e) => {
           // the system clock went backwards!
           println!("Great Scott! {e:?}");
       }
   }
}
