use std::fs;

fn main() {
    let contents = fs::read_to_string("src/bin/resources/4_input.txt")
        .expect("Something went wrong reading the file");

    let entries: Vec<&str> = contents.split("\n\n").collect();

    let hgt = entries
        .into_iter()
        .filter(|x| x.contains("ecl"))
        .filter(|x| x.contains("pid"))
        .filter(|x| x.contains("eyr"))
        .filter(|x| x.contains("hcl"))
        .filter(|x| x.contains("byr"))
        .filter(|x| x.contains("iyr"))
        .filter(|x| x.contains("hgt"))
        .count();

    println!("{}", hgt);
}
