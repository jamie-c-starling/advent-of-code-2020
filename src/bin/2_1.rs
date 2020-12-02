use regex::Regex;
use std::fs;

fn main() {
    let contents = fs::read_to_string("src/bin/resources/2_input.txt")
        .expect("Something went wrong reading the file");

    let contents_str: &str = &contents[..];

    let re = Regex::new(r"([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)").unwrap();

    let mut i = 0;

    for cap in re.captures_iter(contents_str) {
        let desired_letter_count = cap[4].matches(&cap[3]).count() as i32;
        let lower_bound = cap[1].parse::<i32>().unwrap();
        let upper_bound = cap[2].parse::<i32>().unwrap();
        if desired_letter_count <= upper_bound && desired_letter_count >= lower_bound {
            i += 1;
        }
    }

    println!("{}", i);
}
