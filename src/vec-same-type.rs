fn main() {
    // Creating vectors - the macro is convenient for literals
    let numbers = vec![1, 2, 3, 4, 5];

    // Or create an empty one and build it up
    let mut names: Vec<String> = Vec::new();
    names.push(String::from("Alice"));
    names.push(String::from("Bob"));

    // Access elements - panics if out of bounds
    let first = numbers[0];
    println!("\n\tfirst: {}", first);

    // Safe access that returns Option<&T>
    for n in 0..=9 {
        match numbers.get(n) {
            Some(n) => println!("\tFound: {}", n),
            None => println!("\tIndex out of bounds"),
        }
    }
}
