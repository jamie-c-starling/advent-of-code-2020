use regex::Regex;
use std::fs;

fn main() {
    let contents = fs::read_to_string("src/bin/resources/4_input.txt")
        .expect("Something went wrong reading the file");

    let entries: Vec<&str> = contents.split("\n\n").collect();

    let hcl = Regex::new(r"hcl:#([a-f0-9]{6}\b)").unwrap();
    let ecl = Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)\b").unwrap();
    let pid = Regex::new(r"pid:\d{9}\b").unwrap();
    let eyr = Regex::new(r"eyr:([0-9]{4})\b").unwrap();
    let byr = Regex::new(r"byr:([0-9]{4})\b").unwrap();
    let iyr = Regex::new(r"iyr:([0-9]{4})\b").unwrap();
    let hgt = Regex::new(r"hgt:([0-9]{2,3})(cm|in)\b").unwrap();

    let valid = entries
        .into_iter()
        .filter(|x| hcl.is_match(x))
        .filter(|x| ecl.is_match(x))
        .filter(|x| pid.is_match(x))
        .filter(|x| {
            eyr.is_match(x)
                && match eyr.captures(x).unwrap()[1].parse::<i32>().unwrap() {
                    2020..=2030 => true,
                    _ => false,
                }
        })
        .filter(|x| {
            byr.is_match(x)
                && match byr.captures(x).unwrap()[1].parse::<i32>().unwrap() {
                    1920..=2002 => true,
                    _ => false,
                }
        })
        .filter(|x| {
            iyr.is_match(x)
                && match iyr.captures(x).unwrap()[1].parse::<i32>().unwrap() {
                    2010..=2020 => true,
                    _ => false,
                }
        })
        .filter(|x| {
            hgt.is_match(x)
                && match (
                    &hgt.captures(x).unwrap()[2],
                    hgt.captures(x).unwrap()[1].parse::<i32>().unwrap(),
                ) {
                    ("in", 59..=76) => true,
                    ("cm", 150..=193) => true,
                    (_, _) => false,
                }
        })
        .count();

    println!("{}", valid);
}
