use std::io::ErrorKind;
use std::io::Read;
use std::io;
use std::fs::File;

fn create_new_file(file_name: &String) -> File {
    match File::create(file_name) {
        Ok(file) => file,
        Err(error) => {
            panic!("Tried to create file but there was a problem: {:?}", error);
        }
    }
}

fn open_file_using_match(file_name: &String) {
    let f = match File::open(&file_name) {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => create_new_file(&file_name),
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
    };
}

fn open_file_using_unwrap(file_name: &String) {
    let f = File::open(file_name).unwrap();
}

fn open_file_using_expect(file_name: &String) {
    let message = format!("Failed to open {}", file_name);
    let f = File::open(file_name).expect(&message);
}

fn read_username_from_file(file_name: &String) -> Result<String, io::Error> {
    let f = File::open(file_name);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn shortcut(file_name: &String) -> Result<String, io::Error> {
    let mut f = File::open(file_name)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    let file_name = String::from("hello.txt");
    // open_file_using_match(&file_name);
    // open_file_using_unwrap(&file_name);
    // open_file_using_expect(&file_name);

    match read_username_from_file(&file_name) {
        Ok(_) => println!("Success :)"),
        Err(_) => println!("Failed :("),
    }

    match shortcut(&file_name) {
        Ok(_) => println!("Success :)"),
        Err(_) => println!("Failed :("),
    }
}
