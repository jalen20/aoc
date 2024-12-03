use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let file_path = "/Users/jalen/projects/aoc/data/day1.txt";
    let data = read_to_string(file_path).expect("failed to read input");
    let (list_1, list_2) = process_input(data);

    let p1_solution = part_1(list_1.clone(), list_2.clone());
    println!("Part 1 solution: {}", p1_solution);

    let p2_solution = part_2(list_1, list_2);
    println!("Part 2 solution: {}", p2_solution)
}

/// takes in the raw input and returns two sorted vectors.
fn process_input(input: String) -> (Vec<i64>, Vec<i64>) {
    let (mut list_1, mut list_2): (Vec<i64>, Vec<i64>) = input
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                let mut pair = line
                    .split_whitespace()
                    .map(|s| s.parse::<i64>().expect("expected integer value"));

                Some((pair.next().unwrap(), pair.next().unwrap()))
            }
        })
        .unzip();

    list_1.sort_unstable();
    list_2.sort_unstable();

    (list_1, list_2)
}

fn part_1(list_1: Vec<i64>, list_2: Vec<i64>) -> i64 {
    list_1.iter().zip(&list_2).map(|(a, b)| (a - b).abs()).sum()
}

fn part_2(list_1: Vec<i64>, list_2: Vec<i64>) -> i64 {
    let mut acc: HashMap<i64, i64> = HashMap::new();

    for key in list_2 {
        *acc.entry(key).or_insert(0) += 1;
    }

    let result = list_1
        .iter()
        .map(|key| key * acc.get(key).unwrap_or(&0))
        .sum();

    result
}
