use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_report(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

fn find_unsafe_steps(steps: &Vec<i32>) -> Vec<usize> {
    let distances: Vec<i32> = steps.windows(2).map(|e| e[1] - e[0]).collect();

    let sign: i32 = distances[0].signum();

    distances
        .into_iter()
        .enumerate()
        .filter(|t| {
            let val = sign * t.1;
            val <= 0 || val > 3
        })
        .map(|t| t.0)
        .collect()
}

fn is_safe(steps: &Vec<i32>) -> bool {
    find_unsafe_steps(&steps).is_empty()
}

fn is_safe_with_dampener(steps: &Vec<i32>) -> bool {
    for i in 0..steps.len() {
        let mut v = steps.clone();
        v.remove(i);

        if is_safe(&v) {
            return true
        }
    }

    false
}

fn main() {
    let file = File::open("input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let reports: Vec<String> = reader
        .lines()
        .map(|s| s.expect("Failed to read file"))
        .collect();

    let part_1 = reports.iter().filter(|r| is_safe(&read_report(r))).count();
    let part_2 = reports
        .iter()
        .filter(|r| is_safe_with_dampener(&read_report(r)))
        .count();

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST1: [&str; 6] = [
        "7 6 4 2 1",
        "1 2 7 8 9",
        "9 7 6 2 1",
        "1 3 2 4 5",
        "8 6 4 4 1",
        "1 3 6 7 9",
    ];

    #[test]
    fn give_lines_validate_safe_check() {
        // safe
        assert!(is_safe(&read_report(TEST1[0])));
        assert!(is_safe(&read_report(TEST1[5])));

        // unsafe
        assert!(!is_safe(&read_report(TEST1[1])));
        assert!(!is_safe(&read_report(TEST1[2])));
        assert!(!is_safe(&read_report(TEST1[3])));
        assert!(!is_safe(&read_report(TEST1[4])));
    }

    #[test]
    fn give_lines_validate_safe_check_with_dampener() {
        // safe
        assert!(is_safe_with_dampener(&read_report(TEST1[0])));
        assert!(is_safe_with_dampener(&read_report(TEST1[3])));
        assert!(is_safe_with_dampener(&read_report(TEST1[4])));
        assert!(is_safe_with_dampener(&read_report(TEST1[5])));

        // unsafe
        assert!(!is_safe_with_dampener(&read_report(TEST1[1])));
        assert!(!is_safe_with_dampener(&read_report(TEST1[2])));
    }

}
