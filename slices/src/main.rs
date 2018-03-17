fn main() {
    let string = String::from("hello world!");
    let first = first_word(&string[..]);
    let second = second_word(&string);
    println!("first: {}", first);
    println!("second: {}", second);
}

fn first_word(string: &str) -> &str {
    let bytes = string.as_bytes();
    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &string[..index];
        }
    }
    return &string[..];
}

fn second_word(string: &str) -> &str {
    let bytes = string.as_bytes();
    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &string[index + 1..];
        }
    }
    return &string[..];
}
