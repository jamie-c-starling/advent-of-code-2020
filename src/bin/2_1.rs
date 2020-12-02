use std::fs;

struct PasswordAndPolicy {
    required_letter: char,
    lower_bound: i32,
    upper_bound: i32,
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
        lower_bound: bounds[0].parse::<i32>().unwrap(),
        upper_bound: bounds[1].parse::<i32>().unwrap(),
        password: String::from(password),
    }
}

fn is_valid_password(x: PasswordAndPolicy) -> bool {
    let number = x.password.matches(x.required_letter).count() as i32;

    if number >= x.lower_bound && number <= x.upper_bound {
        true
    } else {
        false
    }
}
