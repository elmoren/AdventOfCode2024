use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_line(input: &str) -> usize {
    let pair = input.split_once(": ");
    let (result, operands) = match pair {
        Some((l, r)) => &mut (
            l.parse::<usize>().unwrap(),
            r.split(" ").map(|x| x.parse::<usize>().unwrap()).collect(),
        ),
        None => &mut (0, Vec::new()),
    };

    operands.reverse();
    // println!("Trying: {:?} {:?} {:?}", input, operands, result);
    if resolve(*result, operands) {
        return *result;
    }

    return 0;
}

fn resolve(result: usize, operands: &mut Vec<usize>) -> bool {

    if operands.len() == 1 {
        return result == operands[0];
    }

    let l = operands.pop().unwrap();
    let r = operands.pop().unwrap();

    let mut add = operands.clone();
    add.push(l + r);

    let mut product = operands.clone();
    product.push(l * r);

    let mut concat = operands.clone();
    concat.push(format!("{}{}", l, r).parse::<usize>().unwrap());

    // println!("l, r: {},{}", l, r);
    // println!("Add: {:?} {:?}", result, add);
    // println!("Mul: {:?} {:?}", result, product);
    // println!("Concat: {:?} {:?}", result, concat);

    return resolve(result, &mut add) || resolve(result, &mut product) || resolve(result, &mut concat);
}

fn main() {
    let file = File::open("input.txt").expect("file not found");
    let reader = BufReader::new(file);

    let mut total = 0;
    for line in reader.lines() {
        let l = line.expect("error reading file");
        total += parse_line(&l);
    }

    println!("Part 1 {}", total);
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn given_sample_expect_sum() {
        let total = TEST_INPUT.lines().map(|l| parse_line(l)).sum::<usize>();

        assert_eq!(total, 3749);
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("190: 10 19"), 190);
        assert_eq!(parse_line("3267: 81 40 27"), 3267);
        assert_eq!(parse_line("292: 11 6 16 20"), 292);
    }
}
