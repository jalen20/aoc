use std::fs::read_to_string;

fn main() {
    let file_path = "/Users/jalen/projects/aoc/data/day2.txt";
    let data = read_to_string(file_path).expect("failed to read input");

    let input = process_input(data);

    let p1_solution = part_1(input.clone());
    println!("Part 1 solution: {}", p1_solution);

    let p2_solution = part_2(input);
    println!("Part 2 solution: {}", p2_solution)
}

fn process_input(input: String) -> Vec<Vec<i64>> {
    input
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                let values: Vec<i64> = line
                    .split_whitespace()
                    .map(|s| s.parse::<i64>().expect("expected integer value"))
                    .collect();

                Some(values)
            }
        })
        .collect()
}

fn is_safe_report(report: &[i64]) -> bool {
    if report.len() < 2 {
        return true;
    }

    let mut pairs = report.windows(2);

    if let Some(first_pair) = pairs.next() {
        let first_diff = first_pair[1] - first_pair[0];

        if first_diff.abs() < 1 || first_diff.abs() > 3 {
            return false;
        }

        let sign = first_diff.signum();

        for pair in pairs {
            let diff = pair[1] - pair[0];
            if diff.abs() < 1 || diff.abs() > 3 || diff.signum() != sign {
                return false;
            }
        }
    }
    return true;
}

fn part_1(input: Vec<Vec<i64>>) -> i64 {
    input.iter().filter(|report| is_safe_report(report)).count() as i64
}

fn part_2(input: Vec<Vec<i64>>) -> i64 {
    input
        .iter()
        .map(|report| {
            if is_safe_report(report) {
                return 1 as i64;
            }

            for skip_idx in 0..report.len() {
                let modified_report: Vec<i64> = report
                    .iter()
                    .enumerate()
                    .filter(|(idx, _)| *idx != skip_idx)
                    .map(|(_, &val)| val)
                    .collect();

                if is_safe_report(&modified_report) {
                    return 1 as i64;
                }
            }
            return 0 as i64;
        })
        .sum()
}
