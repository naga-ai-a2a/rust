use std::fmt;

#[derive(Debug)]
struct Point { x: i32, y: i32 }

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y) // Write to the formatter buffer
    }
}

fn main() {
    let p = Point { x: 10, y: 20 };

    // Use {} for Display (user-friendly)
    println!("\n\tDisplay: {}", p);

    // Use {:?} for Debug (programmer-friendly)
    println!("\n\tDebug: {:?}", p);

    // Use {:#?} for "pretty" Debug (adds newlines and indentation)
    println!("\n\tPretty Debug:\n\n{:#?}", p);

    // Using the format! macro to create a String without printing it
    let s = format!("The coordinates are: {}", p);
    println!("\n\tStored String: {}", s);
}
