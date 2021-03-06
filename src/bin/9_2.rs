use std::fs;
use std::process;

fn main() {
    let contents = fs::read_to_string("src/bin/resources/9_input.txt")
        .expect("Something went wrong reading the file");

    let mut lines: Vec<usize> = contents
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let target = 14360655;

    find_sum_to_target(&mut lines, target);
}

fn find_sum_to_target(lines: &mut Vec<usize>, target: usize) {
    let max_index = lines.len() - 1;
    for i in 0..=max_index {
        let mut sum = lines[i];
        for j in i + 1..=max_index {
            sum += lines[j];
            if sum == target {
                let new_vec = &mut lines[i..=j];
                new_vec.sort();
                println!("{}", new_vec[0] + new_vec[new_vec.len() - 1]);
                process::exit(0);
            }
        }
    }
}
