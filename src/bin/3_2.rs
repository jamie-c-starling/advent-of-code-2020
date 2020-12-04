use std::fs;

fn main() {
    let contents = fs::read_to_string("src/bin/resources/3_input.txt")
        .expect("Something went wrong reading the file");

    let hill_rows: Vec<&str> = contents.split("\n").collect();

    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut tree_multiple = 1;
    let tree_symbol: &str = "#";

    for slope in slopes.iter() {
        let mut position_x = 0;
        let mut position_y = 0;
        let mut tree_count = 0;

        for row in hill_rows.iter() {
            let position = position_x % row.len();

            let ch = &row.chars().nth(position).unwrap();

            if position_y % slope.1 == 0 {
                if ch.to_string().eq(tree_symbol) {
                    tree_count += 1;
                }
                position_x += slope.0;
            }
            position_y += 1;
        }
        tree_multiple = tree_multiple * tree_count;
    }
    println!("{}", tree_multiple);
}
