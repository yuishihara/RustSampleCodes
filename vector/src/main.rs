fn print_vector(v: Vec<i32>) {
    println!("vector {:?}", v);
}

fn build_vectors() {
    let v: Vec<i32> = Vec::new();
    print_vector(v);

    let v = vec![1, 2, 3];
    print_vector(v);

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    print_vector(v);
}

fn vector_references() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = v[0];
    v[0] = 5;
    v.push(6);
    println!("{}", first);
    print_vector(v);
}

fn iterate_vector() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    print_vector(v);
}

fn multiple_type_vector() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let v = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    
}

fn main() {
    build_vectors();
    vector_references();
    iterate_vector();
    multiple_type_vector();
}
