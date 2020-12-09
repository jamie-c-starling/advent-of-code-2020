use std::fs;
use std::process;

fn main() {
    let contents = fs::read_to_string("src/bin/resources/9_input.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<usize> = contents
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let mut window: Vec<usize> = Vec::new();
    let window_size = 25;

    for i in 1..=window_size as usize {
        window.push(lines[i - 1]);
    }

    find_spurious_value(&mut window, &lines);
}

fn find_spurious_value(window: &mut Vec<usize>, lines: &Vec<usize>) {
    for x in window.len()..=lines.len() {
        let target = lines[x];
        let mut sums = false;
        for y in window.iter() {
            for z in window.iter() {
                if y + z == target {
                    sums = true;
                }
            }
        }
        if !sums {
            println!("{}", target);
            process::exit(0);
        }
        window.remove(0);
        window.push(target);
    }
}
