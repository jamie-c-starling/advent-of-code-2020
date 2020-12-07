use std::fs;
use std::process;

fn main() {
    let contents = fs::read_to_string("src/bin/resources/8_input.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.lines().collect();

    let mut counter = 0 as i32;
    let mut line_visited: Vec<i32> = Vec::new();

    read_and_act_on_line(0, &lines, &mut counter, &mut line_visited);
}

fn read_and_act_on_line(
    line_num: i32,
    lines: &Vec<&str>,
    counter: &mut i32,
    lines_visited: &mut Vec<i32>,
) -> () {
    if lines_visited.contains(&line_num) {
        println!("{}", counter);
        process::exit(0)
    } else {
        lines_visited.push(line_num);
    };

    let split_line: Vec<&str> = lines[line_num as usize].split(" ").collect();

    let num = split_line[1].parse::<i32>().unwrap();
    match (split_line[0], num) {
        ("nop", _) => {
            read_and_act_on_line(line_num + 1, lines, counter, lines_visited);
        }
        ("acc", x) => {
            *counter += x;
            read_and_act_on_line(line_num + 1, lines, counter, lines_visited);
        }
        ("jmp", x) => read_and_act_on_line(line_num + x, lines, counter, lines_visited),
        _ => (),
    }
}
