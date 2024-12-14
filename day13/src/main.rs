use regex::Regex;

#[derive(Debug)]
struct Game {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

#[derive(Debug)]
struct Mat2 {
    a: (i64, i64),
    b: (i64, i64),
}

fn read_game(input: &str) -> Game {
    let mut iter = input.lines();
    let regex = Regex::new(r"[\s,\+=]").unwrap();
    let a = iter.next().unwrap();
    let b = iter.next().unwrap();
    let prize = iter.next().unwrap();

    let mut splits: Vec<&str>= regex.split(a).collect();
    let a = (splits[3].parse::<i64>().unwrap(), splits[6].parse::<i64>().unwrap());
    splits = regex.split(b).collect();
    let b = (splits[3].parse::<i64>().unwrap(), splits[6].parse::<i64>().unwrap());
    splits = regex.split(prize).collect();
    // let prize = (splits[2].parse::<i64>().unwrap(), splits[5].parse::<i64>().unwrap());
    let prize: (i64, i64) = (10000000000000 + splits[2].parse::<i64>().unwrap(), 10000000000000 + splits[5].parse::<i64>().unwrap());

    Game {
        a,
        b,
        prize
    }
}

fn read_games(input: &str) -> Vec<Game> {
    let mut games = Vec::new();
    let mut tmp = String::new();
    for l in input.lines() {
        if l.is_empty() {
            games.push(read_game(tmp.as_str()));
            tmp.clear();
        } else {
            tmp.push_str(l);
            tmp.push_str("\n");
        }
    }

    games
}

fn solve(coefficients: &Mat2, prize: (i64, i64)) -> Option<(i64, i64)> {
    let x: Mat2 = Mat2 {
        a: (prize.0, coefficients.a.1),
        b: (prize.1, coefficients.b.1),
    };
    let y: Mat2 = Mat2 {
        a: (coefficients.a.0, prize.0),
        b: (coefficients.b.0, prize.1),
    };

    let denom = determinant(&coefficients);
    let mut a_press = determinant(&x);
    let mut b_press = determinant(&y);

    // Check for no solution cases
    if denom == 0 || a_press % denom != 0 || b_press % denom != 0 {
        return None;
    }

    a_press = a_press / denom;
    b_press = b_press / denom;

    return Some((a_press, b_press))
}

fn determinant(mat: &Mat2) -> i64 {
    return (mat.a.0 * mat.b.1) - (mat.a.1 * mat.b.0)
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let games = read_games(&input);

    let result: i64 = games
        .iter()
        .map(|g| {
            let coefficients = Mat2 {
                a: (g.a.0, g.b.0),
                b: (g.a.1, g.b.1),
            };
            let r = solve(&coefficients, g.prize);
            match r {
                Some(r) => r.0 * 3 + r.1,
                None => 0
            }
        })
        .sum();

    println!("Result:  {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    #[test]
    fn test_solve() {
        let simple = Mat2 {
            a: (94, 22),
            b: (34, 67),
        };
        assert_eq!(determinant(&simple), 5550);
        assert_eq!(solve(&simple, (8400, 5400)).unwrap(), (80, 40));
    }

    #[test]
    fn test_solve_2() {
        let simple = Mat2 {
            a: (17, 84),
            b: (86, 37),
        };

        assert_eq!(solve(&simple, (7870, 6450)).unwrap(), (38, 86));
    }

    #[test]
    fn test_solve_fail() {
        let simple = Mat2 {
            a: (26, 67),
            b: (66, 21)
        };

        assert!(solve(&simple, (12748, 12176)).is_none());
    }

    #[test]
    fn test_solve_fail_2() {
        let simple = Mat2 {
            a: (69, 23),
            b: (27, 71)
        };

        assert!(solve(&simple, (12748, 12176)).is_none());
    }


    #[test]
    fn test_single_game() {
        let g = read_game(&SAMPLE);
        let coefficients = Mat2 {
            a: (g.a.0, g.b.0),
            b: (g.a.1, g.b.1),
        };
        assert_eq!(solve(&coefficients, g.prize).unwrap(), (80, 40));
    }

    #[test]
    fn test_sample_input() {
        let games = read_games(&SAMPLE);
        let total: i64 = games
            .iter()
            .map(|g| {
                let coefficients = Mat2 {
                    a: (g.a.0, g.b.0),
                    b: (g.a.1, g.b.1),
                };
                let r = solve(&coefficients, g.prize);
                println!("{:?}", g);
                println!("{:?}", r);
                match r {
                    Some(r) => r.0 * 3 + r.1,
                    None => 0
                }
            })
            .sum();

        assert_eq!(total, 480);
    }
}
