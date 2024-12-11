#[derive(Debug)]
struct TopoMap {
    rows: usize,
    columns: usize,
    map: Vec<i8>,
}

#[derive(Debug)]
struct Trail {
    start: (usize, usize),
    points: Vec<(usize, usize)>,
}

impl TopoMap {
    fn new(input: &str) -> TopoMap {
        let mut iter = input.lines().peekable();
        let columns: usize = iter.peek().unwrap().len();
        let rows: usize = iter.count();
        let map: Vec<i8> = input
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| {
                return match c.to_digit(10) {
                    Some(d) => d as i8,
                    None => -1 as i8,
                };
            })
            .collect();
        TopoMap { columns, rows, map }
    }

    fn get(&self, row: usize, col: usize) -> Option<i8> {
        if !(0..self.rows).contains(&row) || !(0..self.columns).contains(&col) {
            return None;
        }

        Some(self.map[row * self.columns + col])
    }

    fn get_adjacent(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut result = Vec::with_capacity(8);
        let steps = [
            (-1, 0),
            (0, 1),
            (1, 0),
            (0, -1),
        ];

        for (dr, dc) in steps {
            let r = row.checked_add_signed(dr);
            let c = col.checked_add_signed(dc);

            if let (Some(r), Some(c)) = (r, c) {
                result.push((r, c));
            }
        }

        result
    }
}

fn read_trails_at(map: &TopoMap, row: usize, col: usize) -> Trail {
    let mut queue: Vec<(usize, usize)> = vec![(row, col)];
    let mut trails = Vec::new();

    while !queue.is_empty() {
        let (row, col) = queue.pop().unwrap();
        let current = map.get(row, col).unwrap();

        if current == 9 {
            trails.push((row, col));
            continue;
        }

        for (r, c) in map.get_adjacent(row, col).iter() {
            if let Some(val) = map.get(*r, *c) {
                if val == current + 1 {
                    queue.push((*r, *c));
                }
            }
        }
    }

    Trail { start: (row, col), points: trails }
}

fn score_map(map: &TopoMap) -> Vec<Trail> {
    let mut trails = Vec::new();
    for r in 0..map.rows {
        for c in 0..map.columns {
            if let Some(val) = map.get(r, c) {
                if val == 0 {
                    trails.push(read_trails_at(map, r, c));
                }
            }
        }
    }
    trails
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Error reading input.txt");
    let map = TopoMap::new(&input);
    let mut trails = score_map(&map);

    let part2: usize = trails
        .iter()
        .map(|e| {
            e.points.iter().count()
        })
        .sum();

    let part1: usize = trails
        .iter_mut()
        .map(|e| {
            e.points.sort();
            e.points.dedup();
            e.points.iter().count()
        })
        .sum();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_sample_input_part1() {
        let map = TopoMap::new(&INPUT);
        let mut trails = score_map(&map);
        let test: usize = trails
            .iter_mut()
            .map(|e| {
                e.points.sort();
                e.points.dedup();
                e.points.iter().count()
            })
            .sum();
        assert_eq!(test, 36);
    }

    #[test]
    fn test_sample_input_part2() {
        let map = TopoMap::new(&INPUT);
        let trails = score_map(&map);
        let test: usize = trails
            .iter()
            .map(|e| {
                e.points.iter().count()
            })
            .sum();
        assert_eq!(test, 81);
    }
}
