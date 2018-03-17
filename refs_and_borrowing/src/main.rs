fn main() {
    let s1 = String::from("hello");

    let length = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, length);
}

fn calculate_length(s : &String) -> usize { // s is a reference to a String
    s.len()
    // s goes out of scope here. 
    // But because it does not have ownership of what it referes to, nothing happens
}

fn multiple_mutable_refs_with_scope() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    }
    let r2 = &mut s;
}
