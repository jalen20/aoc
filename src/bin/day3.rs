use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let file_path = "/Users/jalen/projects/aoc/data/day3.txt";
    let data = read_to_string(file_path).expect("failed to read input");

    let p1_solution = part_1(&data);
    println!("Part 1 solution: {}", p1_solution);

    let p2_solution = part_2(&data);
    println!("Part 2 solution: {}", p2_solution);
}

fn calculate_products(input: &str) -> i32 {
    let pattern =
        Regex::new(r"mul\((?P<x>\d{1,3}),(?P<y>\d{1,3})\)").expect("failed to compile regex");

    pattern
        .captures_iter(input)
        .filter_map(|cap| {
            let x: i32 = cap.name("x")?.as_str().parse().ok()?;
            let y: i32 = cap.name("y")?.as_str().parse().ok()?;
            Some(x * y)
        })
        .sum()
}

fn part_1(input: &str) -> i32 {
    calculate_products(input)
}

fn part_2(input: &str) -> i32 {
    let removal_pattern =
        Regex::new(r"don't\(\)[\s\S]*?(do\(\)|$)").expect("failed to compile regex");

    let remaining_input = removal_pattern.replace_all(input, "");

    calculate_products(&remaining_input)
}
