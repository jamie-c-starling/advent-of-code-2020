use std::fs;

fn main() {
    let contents = fs::read_to_string("src/bin/resources/10_input.txt")
        .expect("Something went wrong reading the file");

    let mut lines: Vec<usize> = contents
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    lines.sort();

    let mut three = 1;
    let mut one = 0;

    let mut prev = 0 as usize;
    for line in lines.iter() {
        match line - prev {
            1 => one += 1,
            3 => three += 1,
            _ => (),
        }
        prev = line.to_owned();
    }

    println!("{} differences of 3", three);
    println!("{} differences of 1", one);
    println!("product is: {}", one * three);
}
