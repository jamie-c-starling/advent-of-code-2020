use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("src/bin/resources/7_input.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.lines().collect();

    let colour_bag = Regex::new(r"^([\w\s]+) bags contain").unwrap();
    let rules = Regex::new(r"(\d) ([\w\s]+) bags?").unwrap();

    let mut rule_map: HashMap<String, Vec<(String, i32)>> = HashMap::new();

    for line in lines {
        let colour = colour_bag.captures(line).unwrap()[1].to_owned();
        let mut contains: Vec<(String, i32)> = Vec::new();
        for cap in rules.captures_iter(line) {
            contains.push((cap[2].to_owned(), cap[1].to_owned().parse::<i32>().unwrap()));
        }
        rule_map.insert(colour, contains);
    }

    let mut i = 1 as i32;
    let gold = String::from("shiny gold");
    count_bags(&gold, 1, &rule_map, &mut i);

    println!("{}", i - 1);
}

fn count_bags(
    current_colour: &String,
    amount_of_colour: i32,
    rule_map: &HashMap<String, Vec<(String, i32)>>,
    bag_count: &mut i32,
) -> () {
    let contains: &Vec<(String, i32)> = rule_map.get(current_colour).unwrap();

    for i in contains.iter() {
        *bag_count += i.1 * amount_of_colour;
        count_bags(&i.0, i.1 * amount_of_colour, rule_map, bag_count);
    }
}
