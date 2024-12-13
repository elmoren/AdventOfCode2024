use std::collections::HashSet;

#[derive(Debug)]
struct FarmMap {
    rows: usize,
    columns: usize,
    map: Vec<char>,
}

impl FarmMap {
    fn new(input: &str) -> Self {
        let mut iter = input.lines().peekable();
        let columns: usize = iter.peek().unwrap().len();
        let rows: usize = iter.count();
        let map: Vec<char> = input.chars().filter(|c| !c.is_whitespace()).collect();

        FarmMap { rows, columns, map }
    }

    fn contains(&self, row: usize, col: usize) -> bool {
        return (0..self.rows).contains(&row) && (0..self.columns).contains(&col)
    }

    fn get(&self, row: usize, col: usize) -> Option<char> {
        if !self.contains(row, col) {
            return None;
        }

        Some(self.map[row * self.columns + col])
    }

    fn get_neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut result = Vec::with_capacity(4);
        let steps = [(-1, 0), (0, 1), (1, 0), (0, -1)];

        for (dr, dc) in steps {
            let r = row.checked_add_signed(dr);
            let c = col.checked_add_signed(dc);

            if let (Some(r), Some(c)) = (r, c) {
                if self.contains(r, c) {
                    result.push((r, c));
                }
            }
        }

        result
    }
}

#[derive(Debug)]
struct Region {
    crop: char,
    plots: HashSet<(usize, usize)>,
    perimeter: u32,
}

impl Region {
    fn new(map: &FarmMap, row: usize, col: usize) -> Result<Self, String> {
        let crop = map.get(row, col).ok_or("invalid row or col")?;
        let mut region = Region {
            crop,
            plots: HashSet::new(),
            perimeter: 0,
        };
        let mut queue: Vec<(usize, usize)> = vec![(row, col)];
        let mut visited = HashSet::new();
        while !queue.is_empty() {
            let (r, c) = queue.pop().unwrap();
            visited.insert((r, c));
            let current = map.get(r, c).unwrap();

            if current == crop {
                region.add_plot(r, c);
                let neighbors = map.get_neighbors(r, c);
                for (r, c) in neighbors {
                    if !visited.contains(&(r, c)) {
                        queue.push((r, c));
                        visited.insert((r, c));
                    }
                }
            }
        }

        Ok(region)
    }

    fn add_plot(&mut self, row: usize, col: usize) {
        let neighbors = self.count_neighbors(row, col);

        match neighbors {
            0 => self.perimeter += 4,
            1 => self.perimeter += 2,
            2 => (),
            3 => self.perimeter -= 2,
            4 => self.perimeter -= 4,
            _ => unreachable!(),
        }

        self.plots.insert((row, col));
    }

    fn price(&self) -> usize {
        self.perimeter as usize * self.plots.len()
    }

    fn bulk_price(&self) -> usize {
        self.count_corners() * self.plots.len()
    }

    fn count_neighbors(&self, row: usize, col: usize) -> u32 {
        let steps = [(-1, 0), (0, 1), (1, 0), (0, -1)];

        let mut count = 0;
        for (dr, dc) in steps {
            let r = row.checked_add_signed(dr);
            let c = col.checked_add_signed(dc);

            if let (Some(r), Some(c)) = (r, c) {
                if self.plots.contains(&(r, c)) {
                    count += 1;
                }
            }
        }

        count
    }

    fn count_corners(&self) -> usize {
        let mut count = 0;

        let checks = [
            ((0, -1), (-1, -1), (-1, 0)),
            ((-1, 0), (-1, 1), (0, 1)),
            ((0, 1), (1, 1), (1, 0)),
            ((1, 0), (1, -1), (0, -1))
        ];

        for plot in self.plots.iter() {
            for check in checks.iter() {
                let left = self.get_relative(*plot, check.0);
                let middle = self.get_relative(*plot, check.1);
                let right = self.get_relative(*plot, check.2);

                match (left, right) {
                    (None, None) => count += 1,
                    _ => ()
                }

                match (left, middle, right) {
                    (Some(l), None, Some(r)) => count += 1,
                    _ => ()
                };

            }
        }

        count
    }

    fn get_relative(&self, plot: (usize, usize), step: (isize, isize)) -> Option<(usize, usize)> {

        let r = plot.0.checked_add_signed(step.0);
        let c = plot.1.checked_add_signed(step.1);

        if let (Some(r), Some(c)) = (r, c) {
            return self.plots.get(&(r, c)).copied();
        }

        None
    }
}

fn read_regions(map: &FarmMap) -> Vec<Region> {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut regions = Vec::new();

    for i in 0..map.rows {
        for j in 0..map.columns {
            if !visited.contains(&(i, j)) {
                let result = Region::new(map, i, j);
                if result.is_ok() {
                    let region = result.unwrap();
                    visited.extend(&region.plots);
                    regions.push(region);
                }
            }
        }
    }

    regions
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let map = FarmMap::new(&input);
    let regions = read_regions(&map);

    println!("Part 1: {}", regions.iter().map(|r| r.price()).sum::<usize>());
    println!("Part 2: {}", regions.iter().map(|r| r.bulk_price()).sum::<usize>());
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_INPUT: &str = "\
AAAA
BBCD
BBCC
EEEC";

    const WITH_HOLES: &str = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    const COMPLEX: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn test_read_simple() {
        let map = FarmMap::new(SIMPLE_INPUT);
        let region = Region::new(&map, 1, 2).expect("Unable to create region");

        assert_eq!(region.plots.len(), 4);
        assert_eq!(region.perimeter, 10);
    }

    #[test]
    fn test_with_holes() {
        let map = FarmMap::new(WITH_HOLES);
        let regions = read_regions(&map);

        assert_eq!(regions.len(), 5);
        assert_eq!(regions[0].perimeter, 36);
        assert_eq!(regions.iter().map(|r| r.price()).sum::<usize>(), 772);
    }

    #[test]
    fn test_complex_full() {
        let map = FarmMap::new(COMPLEX);
        let regions = read_regions(&map);

        assert_eq!(regions.len(), 11);
        assert_eq!(regions.iter().map(|r| r.price()).sum::<usize>(), 1930);
    }

    #[test]
    fn test_complex_full_bulk() {
        let map = FarmMap::new(COMPLEX);
        let regions = read_regions(&map);

        assert_eq!(regions.iter().map(|r| r.bulk_price()).sum::<usize>(), 1206);
    }

}
