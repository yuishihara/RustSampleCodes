enum IpAddr {
    V4(i32, i32, i32, i32),
    V6(String),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn show_address(ipAddress: IpAddr) {
    match ipAddress {
        IpAddr::V4(first, second, third, last) => {
            println!("{}.{}.{}.{}", first, second, third, last);
        },
        IpAddr::V6(address) => {
            println!("{}", address);
        }
    }
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let ipv4 = IpAddr::V4(255, 255, 255, 254);
    let ipv6 = IpAddr::V6(String::from("abcd::ffff::ffff::ffff"));

    if let IpAddr::V4(..) = ipv4 {
        println!("Oh yeah");
    }

    show_address(ipv4);
    show_address(ipv6);

    println!("Penny in cents: {}", value_in_cents(Coin::Penny));
    println!("Nickel in cents: {}", value_in_cents(Coin::Nickel));
    println!("Dime in cents: {}", value_in_cents(Coin::Dime));
    println!("Quarter in cents: {}", value_in_cents(Coin::Quarter));
}
