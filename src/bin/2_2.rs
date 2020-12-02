use std::fs;

struct PasswordAndPolicy {
    required_letter: char,
    first_char: i32,
    second_char: i32,
    password: String,
}

fn main() {
    let contents = fs::read_to_string("src/bin/resources/2_1_input.txt")
        .expect("Something went wrong reading the file");

    let records = contents.split("\n");

    let mut x = 0;

    for part in records {
        let entry = parse_entry(part);
        if is_valid_password(entry) {
            x += 1;
        }
    }

    println!("{}", x)
}

fn parse_entry(entry: &str) -> PasswordAndPolicy {
    let password_and_policy: Vec<&str> = entry.split(" ").collect();

    let desired_letter: Vec<&str> = password_and_policy[1].split(":").collect();

    let chars: Vec<char> = desired_letter[0].chars().collect();

    let password = password_and_policy[2];
    let bounds: Vec<&str> = password_and_policy[0].split("-").collect();

    PasswordAndPolicy {
        required_letter: chars[0],
        first_char: bounds[0].parse::<i32>().unwrap() - 1,
        second_char: bounds[1].parse::<i32>().unwrap() - 1,
        password: String::from(password),
    }
}

fn is_valid_password(x: PasswordAndPolicy) -> bool {
    let chars: Vec<char> = x.password.chars().collect();
    let mut i = 0;
    let f = x.first_char as usize;
    let s = x.second_char as usize;

    if chars[f] == x.required_letter {
        i += 1;
    }
    if chars[s] == x.required_letter {
        i += 1;
    }

    if i == 1 {
        true
    } else {
        false
    }
}
