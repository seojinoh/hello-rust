use std::any::type_name;
use std::mem::{align_of, size_of};

fn dbg_size<T>() {
    println!("{}: size {} bytes, align: {} bytes",
             type_name::<T>(), size_of::<T>(), align_of::<T>());
}

#[repr(u32)]
enum Foo {
    A,
    B = 10000,
}

fn main() {
    dbg_size::<Foo>();
}