use std::fs;
use std::process;
use std::thread;

fn main() {
    let handler = thread::Builder::new()
        .stack_size(200 * 1024 * 1024)
        .spawn(|| {
            let contents = fs::read_to_string("src/bin/resources/8_input.txt")
                .expect("Something went wrong reading the file");

            let lines: Vec<&str> = contents.lines().collect();

            let mut counter = 0 as i32;
            let mut line_visited: Vec<i32> = Vec::new();
            let mut line_modified = 0 as i32;

            read_and_act_on_line(
                0,
                &lines,
                &mut counter,
                &mut line_visited,
                &mut line_modified,
            );
        })
        .expect("can't spawn thread");

    handler.join().expect("something's wrong with the thread");
}

fn read_and_act_on_line(
    line_num: i32,
    lines: &Vec<&str>,
    counter: &mut i32,
    lines_visited: &mut Vec<i32>,
    line_modified: &mut i32,
) -> () {
    if lines_visited.contains(&line_num) {
        *counter = 0 as i32;
        *lines_visited = vec![];
        *line_modified += 1;
        read_and_act_on_line(0, lines, counter, lines_visited, line_modified)
    }
    if line_num == lines.len() as i32 {
        println!("{}", counter);
        process::exit(0);
    } else {
        lines_visited.push(line_num);
    };

    let split_line: Vec<&str> = lines[line_num as usize].split(" ").collect();

    let num = split_line[1].parse::<i32>().unwrap();
    match (split_line[0], num) {
        ("nop", x) => {
            if line_modified == &line_num {
                read_and_act_on_line(line_num + x, lines, counter, lines_visited, line_modified)
            } else {
                read_and_act_on_line(line_num + 1, lines, counter, lines_visited, line_modified);
            }
        }
        ("acc", x) => {
            *counter += x;
            read_and_act_on_line(line_num + 1, lines, counter, lines_visited, line_modified);
        }
        ("jmp", x) => {
            if line_modified == &line_num {
                read_and_act_on_line(line_num + 1, lines, counter, lines_visited, line_modified);
            } else {
                read_and_act_on_line(line_num + x, lines, counter, lines_visited, line_modified);
            }
        }
        _ => (),
    }
}
