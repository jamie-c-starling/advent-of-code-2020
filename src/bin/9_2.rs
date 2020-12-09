use std::fs;
use std::process;

fn main() {
    let contents = fs::read_to_string("src/bin/resources/9_input.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<usize> = contents
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let target = 14360655;

    find_sum_to_target(&lines, target);
}

fn find_sum_to_target(lines: &Vec<usize>, target: usize) {
    for (i, _) in lines.iter().enumerate() {
        for (j, _) in lines.iter().enumerate() {
            if j > i {
                if lines[i..=j].iter().sum::<usize>() == target {
                    let mut new_vec = lines[i..=j].to_vec();
                    new_vec.sort();
                    println!("{}", new_vec[0] + new_vec[new_vec.len() - 1]);
                    process::exit(0);
                }
            }
        }
    }
}
