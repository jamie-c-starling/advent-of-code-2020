use std::fs;

fn main() {
    let contents = fs::read_to_string("src/bin/resources/6_input.txt")
        .expect("Something went wrong reading the file");

    let groups = contents.split("\n\n");

    let mut dec_count = 0;
    for group in groups {
        let mut declarations: Vec<char> = group.chars().collect();

        declarations.sort();
        declarations.dedup();
        let count = declarations.into_iter().filter(|x| x != &'\n').count();

        dec_count += count;
    }

    println!("{}", dec_count);
}
