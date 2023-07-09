fn main() {
    // Signed integers
    let s1: i8 = -128;
    let s2: i16 = -32768;
    let s3: i32 = -2147483648;
    let s4: i64 = -1_000_000_000_000;
    let s5: i128 = -1_000_000_000_000;
    let s6: isize = -1_000_000_000_000;

    println!("Signed integers: {s1} {s2} {s3} {s4} {s5} {s6}");

    // Unsigned integers
    let u1: u8 = 255;
    let u2: u16 = 65535;
    let u3: u32 = 4294967295;
    let u4: u64 = 1_000_000_000_000;
    let u5: u128 = 1_000_000_000_000;
    let u6: usize = 1_000_000_000_000;

    println!("Unsigned integers: {u1} {u2} {u3} {u4} {u5} {u6}");

    // Floating point numbers
    let f1: f32 = 3.14;
    let f2: f64 = -10.0e20;

    println!("Floating point numbers: {f1} {f2}");

    // Strings
    let str1: &str = "foo";

    println!("Strings: {str1}");

    // Unicode scalar values
    let c1: char = 'a';

    println!("Unicode scalar values: {c1}");

    // Booleans
    let b1: bool = true;

    println!("Booleans: {b1}");

    // Arrays
    let a1: [i8; 3] = [1, 2, 3];
    let mut a2: [i8; 10] = [42; 10];
    a2[5] = 0;

    println!("Arrays: {:?} {:?}", a1, a2);

    // Tuples
    let t1: (i8, bool) = (7, true);
    let mut t2: (i8, bool) = (7, false);
    t2.1 = true;

    println!("Tuples: {:?} {:?}", t1, t2);
}
