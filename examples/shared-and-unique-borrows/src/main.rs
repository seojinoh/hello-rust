fn main() {
    let mut a: i32 = 10;
    let b: &i32 = &a;

    // Error does not occur
    println!("b: {b}");

    {
        let c: &mut i32 = &mut a;
        *c = 20;
    }

    println!("a: {a}");
    // Error occurs
    // println!("b: {b}");
}