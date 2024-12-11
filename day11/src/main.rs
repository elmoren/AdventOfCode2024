use std::collections::HashMap;
use std::ops::Mul;

enum Next {
    AddOne,
    Split,
    Multiply,
}

impl Next {
    fn from_val(num: u64) -> Next {
        if num == 0 {
            return Next::AddOne;
        } else if ((num.checked_ilog10().unwrap_or(0)) + 1) % 2 == 0 {
            return Next::Split;
        }

        return Next::Multiply;
    }

    fn next_val(&self, current: &u64) ->  Vec<u64> {
        match self {
            Next::AddOne => vec!(current + 1),
            Next::Multiply => vec!(current.mul(2024)),
            Next::Split => {
                let mut left = current.to_string();
                let right = left.split_off(left.len() / 2);
                return vec!(left.parse().unwrap(), right.parse().unwrap());
            }
        }
    }
}

fn solve(input: &Vec<u64>, steps: u64) -> u64 {
    let mut rocks: HashMap<u64, u64> = HashMap::new();

    for i in input {
        rocks.insert(*i, 1);
    }

    for _step in 0..steps {
        let to_process: Vec<(u64, u64)> = rocks.drain().collect::<Vec<(u64, u64)>>();
        for rock in to_process {
            let next = Next::from_val(rock.0);
            let result = next.next_val(&rock.0);
            for e in result {
                *rocks.entry(e).or_insert(0) += rock.1;
            }
        }
    }

    rocks.values().sum()
}

fn main() {
    let input: Vec<u64> = vec![4189, 413, 82070, 61, 655813, 7478611, 0, 8];
    println!("Part 1: {}", solve(&input, 25));
    println!("Part 2: {}", solve(&input, 75));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn given_input_expect_solver_total() {
        let input: Vec<u64> = vec![125, 17];
        assert_eq!(solve(&input, 25), 55312);
    }

}
