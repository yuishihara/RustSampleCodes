fn largest(number_list: &Vec<i32>) -> i32 {
    let mut largest = number_list[0];

    for &number in number_list {
        if number > largest {
            largest = number;
        }
    }

    return largest;
}

fn not_duplicated_implementation() {
    let number_list = vec![34, 50, 25, 100, 65];
    println!("The largest number is {}", largest(&number_list));
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    println!("The largest number is {}", largest(&number_list));
}

fn duplicated_implementation() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    // From here duplicated code
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &number in list {
        if largest < number {
            largest = number;
        }
    }
    return largest;
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &character in list {
        if largest < character {
            largest = character;
        }
    }

    return largest;
}

fn largest_anytype<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if largest < item {
            largest = item;
        }
    }

    return largest;
}

fn main() {
    duplicated_implementation();
    not_duplicated_implementation();

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    let result = largest_anytype(&char_list);
    println!("The largest char is {}", result);
}
