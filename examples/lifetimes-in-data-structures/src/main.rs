#[derive(Debug)]
// struct Highlight<'doc>(&'doc str);
struct Highlight<'doc>(&'doc str);

fn erase(text: String) {
    println!("Bye {text}!");
}

fn main() {
    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);
    // Ownership will be lost
    // erase(text);
    println!("{fox:?}");
    println!("{dog:?}");
}