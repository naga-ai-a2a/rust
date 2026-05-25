fn main() {
    println!();

    let v0 = vec!["apple", "banana"];
    println!("v0: {:?}", v0);

    // Using to_string()
    let v1: Vec<String> = v0.iter().map(|s| s.to_string()).collect();
    println!("v1: {:?}", v1);

    // Using String::from (idiomatic alternative)
    let v2: Vec<String> = v0.into_iter().map(String::from).collect();
    println!("v2: {:?}", v2);
}
