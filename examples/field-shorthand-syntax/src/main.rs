#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Person {
    // Same expression
    // fn new(name: String, age: u8) -> Self {
        // Same expression
        //Self { name, age }
        Person { name, age }
    }
}

impl Default for Person {
    fn default() -> Person {
        Person {
            name: "Bot".to_string(),
            age: 0,
        }
    }
}

fn main() {
    let peter = Person::new(String::from("Peter"), 27);
    // Basic representation
    println!("{peter:?}");
    // Debug representation
    println!("{peter:#?}");

    let person = Person {
        ..Person::default()
    };
    println!("{person:#?}");

    let person = Person {
        name: "Sam".to_string(),
        ..Person::default()
    };
    println!("{person:#?}");
}