use std::fs::read_to_string;

fn part_1(input: String) -> i64 {
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

    let result: i64 = list_1.iter().zip(&list_2).map(|(a, b)| (a - b).abs()).sum();

    result
}

fn main() {
    let file_path = "/Users/jalen/projects/aoc/aoc_24/data/day1.txt";
    let data = read_to_string(file_path).expect("failed to read input");
    let p1_solution = part_1(data);
    println!("{:?}", p1_solution)
}
