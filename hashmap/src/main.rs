use std::collections::HashMap;

fn hashmap_from_insert() -> HashMap<String, i32> {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);

    return scores;
}

fn hashmap_from_tuples() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("{:?}", scores);
}

fn hash_map_and_ownership() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    println!("field_name: {}, field_value: {}", field_name, field_value);

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    println!("{:?}", map);
}

fn accessing_hash_map_values() {
    let scores = hashmap_from_insert();
    let team_name = String::from("Blue");

    let score = scores.get(&team_name);

    match score {
        Some(x) => println!("score of {} is {}", team_name, x),
        None => println!("Score map does not contain value for team: {}", &team_name)
    }
}

fn overwrite_value() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("Overwritten score: {:?}", scores);
}

fn insert_if_does_not_exist() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yesllow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("Inserted if does not exist {:?}", scores);
}

fn update_based_on_old_value() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("map updated based on old value: {:?}", map)
}

fn main() {
    hashmap_from_insert();
    hashmap_from_tuples();
    hash_map_and_ownership();
    accessing_hash_map_values();
    overwrite_value();
    insert_if_does_not_exist();
    update_based_on_old_value();
}
