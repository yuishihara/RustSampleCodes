use std::{i16, i32, i64, i8, u16, u32, u64, u8};

fn main() {
    println!("Hello, world!");
    println!("i8 min: {}, max: {}", i8::MIN, i8::MAX);
    println!("i16 min: {}, max: {}", i16::MIN, i16::MAX);
    println!("i32 min: {}, max: {}", i32::MIN, i32::MAX);
    println!("i64 min: {}, max: {}", i64::MIN, i64::MAX);
    println!("u8 min: {}, max: {}", u8::MIN, u8::MAX);
    println!("u16 min: {}, max: {}", u16::MIN, u16::MAX);
    println!("u32 min: {}, max: {}", u32::MIN, u32::MAX);
    println!("u64 min: {}, max: {}", u64::MIN, u64::MAX);

    let it_is_true: bool = true;
    let let_x: char = 'x';
    println!("it is {0} that {1} is {0}", it_is_true, let_x);
    println!("{:.2}", 1.234);

    println!("B {:b}, H {:x}, O {:o}", 10, 10, 10);

    println!("{ten:>ws$}", ten = 10, ws = 5);
    println!("{ten:>0ws$}", ten = 10, ws = 5);
}
