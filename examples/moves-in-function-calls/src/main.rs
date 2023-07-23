fn say_hello(name: String) {
    println!("Hello {name}")
}

fn say_hello_with_ref(name: &String) {
    println!("Hello {name}")
}

fn main() {
    let name = String::from("Alice");
    say_hello(name.clone());
    say_hello_with_ref(&name);
    say_hello(name);
}