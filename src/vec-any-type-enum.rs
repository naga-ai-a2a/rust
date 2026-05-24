#[derive(Debug)]
enum Mixed {
    Int(i32),
    Text(String),
    Float(f64),
}

fn main() {
    // A vector that can hold any of the variants defined in the Mixed enum
    let list: Vec<Mixed> = vec![
        Mixed::Int(42),
        Mixed::Text("Hello Rust".to_string()),
        Mixed::Float(3.14),
    ];

    for item in list {
        match item {
            Mixed::Int(i) => println!("\nInteger: {i}"),
            Mixed::Text(s) => println!("\nText: {s}"),
            Mixed::Float(f) => println!("\nFloat: {f}"),
        }
    }
}
