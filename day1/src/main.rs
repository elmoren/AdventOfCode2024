use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};


fn sum_distances<'a, I>(left: I, right: I) -> i32 
where 
    I: IntoIterator<Item = &'a i32>,
{
    left
        .into_iter()
        .zip(right.into_iter())
        .map(|t| (t.1 - t.0).abs())
        .sum()
}

fn sum_similarity<'a, I>(left: I, right: I) -> i32 
where 
    I: IntoIterator<Item = &'a i32>,
{
    let mut frequencies: HashMap<i32, i32> = HashMap::new();
    for e in right.into_iter() {
        *frequencies.entry(*e).or_insert(0) += 1;
    }

    return left
        .into_iter()
        .map(|v| {
            match frequencies.get(v) {
                None => 0, 
                Some(ct) => v * *ct
            }
        })
        .sum();
}

fn main() {
    let file = File::open("input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut left = Vec::new();
    let mut right = Vec::new();


    for line in reader.lines() {
        let l = line.expect("Unable to read file");

        let mut res = l.split_whitespace();
        let l: i32 = res.next().unwrap().parse().unwrap();
        let r: i32 = res.next().unwrap().parse().unwrap();

        left.push(l);
        right.push(r);

    }

    left.sort();
    right.sort();

    let part_one = sum_distances(&left, &right);
    let part_two = sum_similarity(&left, &right);

    println!("Part 1: {}", part_one);
    println!("Part 2: {}", part_two);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn given_sample_score_part_1() {
        let mut left_1 = Vec::from([3, 4, 2, 1, 3, 3]);
        let mut right_1 = Vec::from([4, 3, 5, 3, 9, 3]);

        left_1.sort();
        right_1.sort();

        let score = sum_distances(&left_1, &right_1);
        assert_eq!(score, 11)
    }
    
    #[test]
    fn given_sample_score_part_2() {
        let mut left_1 = Vec::from([3, 4, 2, 1, 3, 3]);
        let mut right_1 = Vec::from([4, 3, 5, 3, 9, 3]);

    
        let score = sum_similarity(&left_1, &right_1);
        assert_eq!(score, 31)
    }

}