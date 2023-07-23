fn takes_u32(x: u32) {
    println!("u32: {x}");
}

fn takes_i8(y: i8) {
    println!("i8: {y}");
}

fn print_type_of<T>(_: T) {
    println!("Type is: {}", std::any::type_name::<T>())
}

fn main() {
    let x = 10;
    let y = 20;

    takes_u32(x);
    print_type_of(&x);
    takes_i8(y);
    print_type_of(&y);

    let mut v = Vec::new();
    v.push((10, false));
    v.push((20, true));
    println!("v: {v:?}");
    print_type_of(&v);

    let vv = v.iter().collect::<std::collections::HashSet<_>>();
    println!("vv: {vv:?}");
    print_type_of(&vv);
}