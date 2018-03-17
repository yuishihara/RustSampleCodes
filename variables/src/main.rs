fn calculations() {
    let sum = 5 + 10;
    println!("sum: {}", sum);

    let difference = 95.5 - 4.3;
    println!("difference: {}", difference);

    let product = 4 * 30;
    println!("product: {}", product);

    let quotient = 56.7 / 32.2;
    println!("quotient: {}", quotient);

    let remainder = 43 % 5;
    println!("remainder: {}", remainder);
}

fn tuples() -> (i32, u8) {
    return (10_000, 255);
}

fn main() {
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS: {}", MAX_POINTS);

    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is: {}", x);

    calculations();

    let (big, small) = tuples();
    println!("big: {big}, small: {small}", big = big, small = small);
}
