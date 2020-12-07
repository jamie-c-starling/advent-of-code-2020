use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("src/bin/resources/7_input.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.lines().collect();

    let colour_bag = Regex::new(r"^([\w\s]+) bags contain").unwrap();
    let rules = Regex::new(r"(\d) ([\w\s]+) bags?").unwrap();

    let mut rule_map: HashMap<String, Vec<String>> = HashMap::new();

    for line in lines {
        let colour = colour_bag.captures(line).unwrap()[1].to_owned();
        let mut contains: Vec<String> = Vec::new();
        for cap in rules.captures_iter(line) {
            contains.push(cap[2].to_owned());
        }
        rule_map.insert(colour, contains);
    }

    let mut c_g_list: Vec<String> = Vec::new();
    for (k, _) in &rule_map {
        find_gold(k, k, &rule_map, &mut c_g_list);
    }

    println!("{}", c_g_list.len());
}

fn find_gold(
    starting_colour: &String,
    current_colour: &String,
    rule_map: &HashMap<String, Vec<String>>,
    already_found: &mut Vec<String>,
) -> () {
    let gold = String::from("shiny gold");
    let contains: &Vec<String> = rule_map.get(current_colour).unwrap();
    if contains.contains(&gold) && !already_found.contains(starting_colour) {
        already_found.push(starting_colour.to_owned());
    } else {
        for i in contains.iter() {
            find_gold(starting_colour, i, rule_map, already_found)
        }
    }
}
