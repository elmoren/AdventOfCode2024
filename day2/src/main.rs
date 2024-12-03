use std::fs::File;
use std::io::{BufRead, BufReader};

fn count_safe(reports: &Vec<String>) -> usize {
    reports
        .iter()
        .filter(|r| is_safe(*r))
        .count()
}

fn is_safe(line: &str) -> bool {
    let list = line.split_whitespace().collect::<Vec<&str>>();

    let distances: Vec<i32> = list
        .windows(2)
        .map(|e: &[&str]| e[1].parse::<i32>().unwrap() - e[0].parse::<i32>().unwrap())
        .collect();

    let sign: i32 = distances[0].signum();

    distances.into_iter().all(|d| {
        let val = sign * d;
        val > 0 && val <= 3
    })
}

fn main() {
    let file = File::open("input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let reports: Vec<String> = reader
        .lines()
        .map(|s| s.expect("Failed to read file"))
        .collect();

    let part_1 = count_safe(&reports);

    println!("Part 1: {}", part_1);
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
        assert!(is_safe(TEST1[0]));
        assert!(is_safe(TEST1[5]));

        // unsafe
        assert!(!is_safe(TEST1[1]));
        assert!(!is_safe(TEST1[2]));
        assert!(!is_safe(TEST1[3]));
        assert!(!is_safe(TEST1[4]));
    }

    #[test]
    fn give_test1_expect_2() {
        let mut reports: Vec<String> = Vec::new();

        for line in TEST1 {
            reports.push(String::from(line));
        }

        let r = count_safe(&reports);

        assert_eq!(r, 2)
    }
}
