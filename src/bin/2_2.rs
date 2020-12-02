use regex::Regex;
use std::fs;

fn main() {
    let contents = fs::read_to_string("src/bin/resources/2_input.txt")
        .expect("Something went wrong reading the file");

    let contents_str: &str = &contents[..];

    let re = Regex::new(r"([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)").unwrap();

    let mut i = 0;

    for cap in re.captures_iter(contents_str) {
        let first_position = (cap[1].parse::<i32>().unwrap() - 1) as usize;
        let second_position = (cap[2].parse::<i32>().unwrap() - 1) as usize;
        let desired_chars: Vec<char> = cap[3].chars().collect();
        let chars: Vec<char> = cap[4].chars().collect();

        if chars[first_position] == desired_chars[0] || chars[second_position] == desired_chars[0] {
            if chars[first_position] != chars[second_position] {
                i += 1;
            }
        }
    }
    println!("{}", i);
}
