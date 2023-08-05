enum Result {
    Ok(i32),
    Zero(String),
    Err(String)
}

fn divide_in_two(n: i32) -> Result {
    if n == 0 {
        Result::Zero(format!("{n} is 0"))
    } else if n % 2 == 0 {
        Result::Ok(n / 2)
    } else {
        Result::Err(format!("cannot divide {n} into two equal parts"))
    }
}

fn main() {
    let n = 0;
    match divide_in_two(n) {
        Result::Ok(half) => println!("{n} divided in two is {half}"),
        Result::Zero(msg) => println!("{n} is 0 {msg}"),
        Result::Err(msg) => println!("sorry, an error happened: {msg}"),
    }
}