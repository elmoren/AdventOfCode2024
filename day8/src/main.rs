use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, Hash, PartialEq)]
struct Point {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct Map {
    rows: usize,
    cols: usize,
    antennas: HashMap<char, Vec<Point>>,
    antinodes: HashSet<Point>,
}

fn read_map(input: &str) -> Map {
    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
    let mut antinodes = HashSet::new();
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();

    input.lines().enumerate().for_each(|(row, line)| {
        let chars = line.chars();
        for (col, c) in chars.enumerate() {
            if c != '.' {
                let p = Point { row, col };
                let opts = antennas.get(&c);
                match opts {
                    Some(existing) => {
                        let nodes: Vec<Point> = find_antinodes(existing, &p, rows, cols);
                        nodes.into_iter().for_each(|p| {
                            antinodes.insert(p);
                        });
                    }
                    None => {}
                }

                antennas
                    .entry(c)
                    .or_insert(Vec::new())
                    .push(Point { row, col });
            }
        }
    });

    Map {
        rows,
        cols,
        antennas,
        antinodes,
    }
}

fn find_antinodes(antennas: &Vec<Point>, p: &Point, rows: usize, cols: usize) -> Vec<Point> {
    let mut anodes = Vec::new();
    for a in antennas {
        let n1 = calc_antinode(a, p, rows as isize, cols as isize);
        let n2 = calc_antinode(p, a, rows as isize, cols as isize);

        if let Some(tmp) = n1 {
            anodes.push(tmp);
        }

        if let Some(tmp) = n2 {
            anodes.push(tmp);
        }
    }
    return anodes;
}

fn calc_antinode(p1: &Point, p2: &Point, max_rows: isize, max_cols: isize) -> Option<Point> {
    let dx = p2.row as isize - p1.row as isize;
    let dy = p2.col as isize - p1.col as isize;

    let tmp = (p2.row as isize + dx, p2.col as isize + dy);

    if (0..max_rows).contains(&tmp.0) && (0..max_cols).contains(&tmp.1) {
        return Some(Point {
            row: tmp.0 as usize,
            col: tmp.1 as usize,
        });
    }

    return None;
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Error reading input.txt");
    let map = read_map(&input);

    println!("Part 1: {}", map.antinodes.len())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part1() {
        let m = read_map(TEST_INPUT);

        println!("{:#?}", m);
        assert_eq!(m.antinodes.len(), 14);
        assert_eq!(m.antennas.len(), 2);
        assert_eq!(m.rows, 12);
        assert_eq!(m.cols, 12);
    }

    #[test]
    fn test_calc_antinodes() {
        assert_eq!(
            calc_antinode(&Point { row: 2, col: 2 }, &Point { row: 4, col: 3 }, 10, 10),
            Some(Point { row: 6, col: 4 })
        );

        assert_eq!(
            calc_antinode(&Point { row: 4, col: 3 }, &Point { row: 2, col: 2 }, 10, 10),
            Some(Point { row: 0, col: 1 })
        );

        assert_eq!(
            calc_antinode(&Point { row: 4, col: 3 }, &Point { row: 2, col: 2 }, 10, 10),
            Some(Point { row: 0, col: 1 })
        );

        assert_eq!(
            calc_antinode(&Point { row: 3, col: 3 }, &Point { row: 6, col: 6 }, 10, 10),
            Some(Point { row: 9, col: 9 })
        );

        assert_eq!(
            calc_antinode(&Point { row: 3, col: 7 }, &Point { row: 6, col: 3 }, 10, 10),
            None
        );

        assert_eq!(
            calc_antinode(&Point { row: 8, col: 8 }, &Point { row: 2, col: 2 }, 10, 10),
            None
        );

        assert_eq!(
            calc_antinode(&Point { row: 2, col: 2 }, &Point { row: 8, col: 8 }, 10, 10),
            None
        );
    }
}
