fn push_string() {
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);
}

fn concatenate_strings() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(&s2);

    println!("s1 is {}", s1);
    println!("s2 is {}", s2);
}

fn add_character() {
    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);
}

fn plus_operator() {
    let s1 = String::from("Hello, ");
    println!("s2: {}", s1);

    let s2 = String::from("world!");
    println!("s2: {}", &s2);

    let s3 = s1 + &s2;
    println!("s3: {}", s3);
}

fn format_macro() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
}

fn indexing() {
    let s1 = String::from("hello");
    let h = s1[0];
}

fn main() {
    push_string();
    concatenate_strings();
    add_character();
    plus_operator();
    format_macro();
    indexing();
}
