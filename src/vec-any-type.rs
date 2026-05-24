trait Speaker {
    fn speak(&self);
}

struct Dog;
impl Speaker for Dog {
    fn speak(&self) { println!("\nWoof!"); }
}

struct Cat;
impl Speaker for Cat {
    fn speak(&self) { println!("\nMeow!"); }
}

fn main() {
    // Box<dyn Speaker> allows the vector to store any type implementing Speaker
    let critters: Vec<Box<dyn Speaker>> = vec![
        Box::new(Dog),
        Box::new(Cat),
    ];

    for critter in critters {
        critter.speak();
    }
}
