fn main() {
    let v: Vec<String> = vec![String::from("foo"), String::from("bar")];

    for word in &v {
        println!("┌ word: {}", word);
        println!("└ type: {}", print_type_of(word));
    }

    for word in v {
        println!("┌ word: {}", word);
        println!("└ type: {}", print_type_of(word));
    }
}

fn print_type_of<T>(_: T) -> String {
    std::any::type_name::<T>().to_string()
}