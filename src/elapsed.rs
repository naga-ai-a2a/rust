fn main() {
   let now = std::time::SystemTime::now();

   // we sleep for 2 seconds
   std::thread::sleep(std::time::Duration::new(2, 0));

   match now.elapsed() {
       Ok(elapsed) => {
           // it prints '2'
           println!("\n\tElapsed time: {} seconds", elapsed.as_secs());
       }
       Err(e) => {
           // the system clock went backwards!
           println!("Great Scott! {e:?}");
       }
   }
}
