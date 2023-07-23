fn main() {
    let s1: String = String::from("Hello!");
    let s2: String = s1;
    println!("s2: {s2}");
    // Error occurs
    // println!("s1: {s1}");

    let s3: String = String::from("Hello!");
    {
        let s4: String = s3;
        println!("s4: {s4}");
    }
    // Error occurs
    // println!("s3: {s3}");
}