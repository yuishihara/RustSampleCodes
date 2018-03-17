fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s2); // s1 can't be used here. ownership is already moved to s2

    let s1 = gives_ownership();
    println!("gives_ownership: {}", s1);

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    
    println!("takes and gives back: {}", s3);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    return a_string;
}
