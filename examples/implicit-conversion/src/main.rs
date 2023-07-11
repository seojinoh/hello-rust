fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

fn main() {
    let x: i8 = 15;
    let y: i16 = 1000;

    // 방법 1:
    // println!("{x} * {y} = {}", multiply(x.into(), y));
    // 방법 2:
    println!("{x} * {y} = {}", multiply(i16::from(x), y));
}
