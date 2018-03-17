fn main() {
    // danger_infinite_loop();
    while_loop();
    for_loop();
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}:", number);
    }
    println!("LIFTOFF!!!");
}

fn while_loop() {
    let mut number = 3;
    while number != 0 {
        println!("{}:", number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");
}

fn danger_infinite_loop() {
    loop {
        println!("again!");
    }
}
