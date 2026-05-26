use std::error::Error;

fn do_something() -> Result<(), Box<dyn Error>> {
    let _file = std::fs::File::open("config.txt")?; // Automatically boxes io::Error
    Ok(())
}

fn manual_error() -> Result<(), Box<dyn Error>> {
    return Err("something went wrong".into()); 
}

// #[derive(Debug)]
// struct MyError;
// impl std::fmt::Display for MyError { Err("An error has occurred") }
// impl std::error::Error for MyError {}

// fn custom_box() -> Result<(), Box<dyn Error>> {
//     Err(Box::new(MyError))
// }

fn main() {
    println!("\n\t{:?}", do_something());
    println!("\t{:?}", manual_error());
}
