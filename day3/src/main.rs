use std::io::{BufRead, BufReader};
use std::fs::File;

fn find_mul(buffer: &str) -> i32 {
    let candidates: Vec<usize> = buffer.match_indices("mul(").map(|d| d.0).collect();

    let result: Vec<Option<i32>> = candidates
        .iter()
        .map(|e| parse_cmd(&buffer[e+4..]))
        .collect();

    return result
        .iter()
        .map(|r| r.unwrap_or(0))
        .sum()
}

fn parse_cmd(buf: &str) -> Option<i32>{

    let mut chars = buf.chars().peekable();
    let mut work_buf = String::new();

    while let Some(c) = chars.next_if(|n| n.is_digit(10)) {
        work_buf.push(c);
    }
    println!("Work Buf 1: {}", work_buf);

    let v1 = work_buf.parse::<i32>().ok();
    if v1.is_none() {
        return None;
    }

    if chars.peek() == Some(&',') {
        chars.next();
    } else {
        return None
    }

    work_buf.clear();
    while let Some(c) = chars.next_if(|n| n.is_digit(10)) {
        work_buf.push(c);
    }
    println!("Work Buf 2: {}", work_buf);

    let v2 = work_buf.parse::<i32>().ok();
    if v2.is_none() {
        return None
    }

    if chars.peek() != Some(&')') {
        return None;
    }

    println!("{:?} * {:?}", v1, v2);
    return Some(v1.unwrap() * v2.unwrap());
}

fn main() {

    let file = File::open("input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut total = 0;

    for line in reader.lines() {
        let line = line.expect("Read error");
        println!("Line: {}", line);
        total += find_mul(&line);
    }

    println!("Part 1: {}", total);

}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn given_sample_expect_161() {
        let r = find_mul(SAMPLE);
        assert_eq!(r, 161);
    }

    #[test]
    fn given_sample_expect_parsed() {
        let r = parse_cmd(&SAMPLE[5..]);
    }

}