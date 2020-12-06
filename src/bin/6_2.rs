use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("src/bin/resources/6_input.txt")
        .expect("Something went wrong reading the file");

    let groups = contents.split("\n\n");

    let mut dec_count = 0;
    for group in groups {
        let mut occurances: HashMap<char, i32> = HashMap::new();

        let declarations: Vec<&str> = group.lines().collect();
        for declarations in &declarations {
            let chars: Vec<char> = declarations.chars().collect();
            for char in chars {
                *occurances.entry(char).or_insert(0) += 1;
            }
        }
        let group_count = occurances
            .into_iter()
            .filter(|&(_, v)| v == declarations.len() as i32)
            .count();

        dec_count += group_count;
    }
    println!("{}", dec_count);
}
