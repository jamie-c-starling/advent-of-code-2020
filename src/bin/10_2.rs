use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("src/bin/resources/10_input.txt")
        .expect("Something went wrong reading the file");

    let mut lines: Vec<usize> = contents
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    lines.push(0);

    lines.sort();

    let mut cache: HashMap<usize, usize> = HashMap::new();

    println!(
        "{}",
        calculate_combinations(0, &lines, lines.len(), &mut cache)
    );
}

fn calculate_combinations(
    i: usize,
    adaptors: &Vec<usize>,
    length: usize,
    cache: &mut HashMap<usize, usize>,
) -> usize {
    if i == length - 1 {
        return 1;
    }
    if cache.contains_key(&i) {
        return cache.get(&i).unwrap().to_owned();
    }
    let mut sum = 0;
    for j in (i + 1)..length {
        if adaptors[j] - adaptors[i] <= 3 {
            sum += calculate_combinations(j, adaptors, length, cache);
        }
    }
    cache.insert(i, sum);
    return sum;
}
