use std::fs;

fn main() {
    let contents = fs::read_to_string("src/bin/resources/3_input.txt")
        .expect("Something went wrong reading the file");

    let hill_rows = contents.split("\n");

    let mut x = 0;
    let mut tree_count = 0;
    let tree_symbol: &str = "#";

    for row in hill_rows {
        let position = x % row.len();

        let ch = row.chars().nth(position).unwrap();

        if ch.to_string().eq(tree_symbol) {
            tree_count += 1;
        }
        x += 3;
    }
    println!("{}", tree_count);
}
