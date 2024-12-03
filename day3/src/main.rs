use std::fs;

fn find_products(buffer: &str) -> i32 {
    let candidates: Vec<usize> = buffer.match_indices("mul(").map(|d| d.0).collect();

    let result: Vec<Option<i32>> = candidates
        .iter()
        .map(|e| parse_mul(&buffer[e+4..]))
        .collect();

    return result
        .iter()
        .map(|r| r.unwrap_or(0))
        .sum()
}

fn find_products_with_conditionals(buffer: &str) -> i32 {
    let chars = buffer.chars();
    let mut processing = true;
    let mut work_buf: String = String::new();
    let mut result: Vec<i32> = Vec::new();

    for (i, c) in chars.enumerate() {
        work_buf.push(c);

        if work_buf.ends_with("do()") {
            processing = true;
            work_buf.clear();
        } else if work_buf.ends_with("don't()") {
            processing = false;
            work_buf.clear();
        } else if work_buf.ends_with("mul(") && processing {
            work_buf.clear();
            result.push(parse_mul(&buffer[i+1..]).unwrap_or(0));
        }
    }

    return result
        .iter()
        .sum()
}

fn parse_mul(buf: &str) -> Option<i32>{

    let mut chars = buf.chars().peekable();
    let mut work_buf = String::new();

    while let Some(c) = chars.next_if(|n| n.is_digit(10)) {
        work_buf.push(c);
    }

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

    let v2 = work_buf.parse::<i32>().ok();
    if v2.is_none() {
        return None
    }

    if chars.peek() != Some(&')') {
        return None;
    }

    return Some(v1.unwrap() * v2.unwrap());
}

fn main() {
    let input: String = fs::read_to_string("input.txt").expect("err");

    let mut part_1 = 0;
    let mut part_2 = 0;

    part_1 = find_products(&input);
    part_2 = find_products_with_conditionals(&input);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const SAMPLE2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn given_sample_expect_161() {
        let r = find_products(SAMPLE);
        assert_eq!(r, 161);
    }

    #[test]
    fn given_sample_with_conditionals_expect_48() {
        let r = find_products_with_conditionals(SAMPLE2);
        assert_eq!(r, 48);
    }

}