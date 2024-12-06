use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct PuzzleMap {
    rows: usize,
    cols: usize,
    obstructions: HashMap<(usize, usize), bool>,
    visited: HashMap<(usize, usize), usize>,
    start: (usize, usize, Direction),
    guard: (usize, usize, Direction),
    history: Vec<(usize, usize, Direction)>
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl PuzzleMap {
    fn reset(&mut self) {
        self.guard = (self.start.0, self.start.1, self.start.2.clone());
        self.history.clear();
        self.visited.clear();
    }

    fn part_1(&mut self) -> Result<(), ()> {
       loop {
           let next_opt = match self.guard.2 {
               Direction::Up => (self.guard.0.checked_sub(1), Some(self.guard.1)),
               Direction::Right => (Some(self.guard.0), self.guard.1.checked_add( 1)),
               Direction::Down => (self.guard.0.checked_add(1), Some(self.guard.1)),
               Direction::Left => (Some(self.guard.0), self.guard.1.checked_sub(1)),
           };

           if next_opt.0.is_none() || next_opt.1.is_none() {
               return Ok(());
           }

           let next = (next_opt.0.unwrap(), next_opt.1.unwrap());
           if !(0..self.rows).contains(&next.0) || !(0..self.cols).contains(&next.1) {
               return Ok(());
           }

           if self.history.contains(&(next.0, next.1, self.guard.2.clone())) {
               return Err(())
           }

           let obs = self.obstructions.get(&next);
           match obs {
               Some(_obs) => self.guard.2 = self.next_dir(),
               None => {
                   self.guard.0 = next.0;
                   self.guard.1 = next.1;
                   *self.visited.entry(next).or_insert(0) += 1;
                   self.history.push(self.guard.clone());
               }
           }
       }
    }

    fn part_2(&mut self) -> usize {
        let keys: Vec<(usize, usize)> = self.visited.keys().map(|k| (k.0, k.1)).collect();
        let mut total: usize = 0;

        for k in keys {
            self.reset();

            self.obstructions.insert(k, true);
            let r = self.part_1();
            total += match r {
                Ok(()) => 0,
                Err(()) => 1
            };
            self.obstructions.remove(&k);
        }
        return total;
    }

    fn next_dir(&mut self) -> Direction {
        return match self.guard.2 {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up
        }
    }
}

fn init_map(input: Vec<&str>) -> PuzzleMap {
    let mut obstructions: HashMap<(usize, usize), bool> = HashMap::new();
    let mut visited: HashMap<(usize, usize), usize> = HashMap::new();
    let mut history: Vec<(usize, usize, Direction)> = Vec::with_capacity(10000);
    let rows = input.len();
    let cols = input[0].len();
    let mut guard= (0, 0, Direction::Up);

    input
        .iter()
        .enumerate()
        .for_each(|(row, line)| {
                line.
                    chars()
                    .enumerate()
                    .for_each(
                        |(col, chr)| {
                            match chr {
                                '^' => guard = (row, col, Direction::Up),
                                '>' => guard = (row, col, Direction::Right),
                                'v' => guard = (row, col, Direction::Down),
                                '<' => guard = (row, col, Direction::Left),
                                '#' => {obstructions.insert((row, col), true);},
                                _ => ()
                            }
                        }
                    );
            }
        );

    visited.insert((guard.0, guard.1), 1);
    history.push(guard.clone());

    return PuzzleMap {
        rows,
        cols,
        obstructions,
        visited,
        start: (guard.0, guard.1, guard.2.clone()),
        guard,
        history
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("File does not exist");
    let mut puzzle_map = init_map(input.lines().collect());

    let _ = puzzle_map.part_1();
    let part_2 = puzzle_map.part_2();
    println!("Part 1: {}", puzzle_map.visited.len());
    println!("Part 2 : {}", part_2);
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn given_sample_part_1_expect_41(){
        let mut puzzle_map = init_map(TEST_INPUT.lines().collect());

        // println!("{:?}", puzzle_map.);
        let _ = puzzle_map.part_1();
        assert_eq!(puzzle_map.visited.len(), 41);
    }

    #[test]
    fn given_sample_part_2_expect_6(){
        let mut puzzle_map = init_map(TEST_INPUT.lines().collect());

        let _ = puzzle_map.part_1();
        assert_eq!(puzzle_map.part_2(), 6);
    }

}