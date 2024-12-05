struct Puzzle {
    rows: usize,
    columns: usize,
    puzzle: Vec<char>
}

trait CrosswordPuzzle {
    fn char_at(&self, r: usize, c: usize) -> Option<&char>;

    fn find_words(&self, word: &str) -> usize;

    fn find_x_mas(&self) -> usize;

    fn match_word(&self, word_slice: &str, row: usize, col: usize, r_step: isize, c_step: isize) -> bool;
}

impl CrosswordPuzzle for Puzzle {
    fn char_at(&self, r: usize, c: usize) -> Option<&char> {
        if !(0..self.rows).contains(&r) || !(0..self.columns).contains(&c) {
            return None;
        }

        return self.puzzle.get(r * self.columns + c);
    }

    fn find_words(&self, word: &str) -> usize {
        let mut matches = 0;
        let steps = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for r in 0..self.rows {
            for c in 0..self.columns {
                for p in steps.iter() {
                    if self.match_word(word, r, c, p.0, p.1) {
                        matches += 1;
                    }
                }
            }
        }

        return matches;
    }

    // Create a set of 3x3 filters in 1d string slice for each orientation. '.' are ignored
    // Apply the filters to the puzzle and count matches
    fn find_x_mas(&self) -> usize {
        let mut matches = 0;
        let filters = [
            "M.M.A.S.S",
            "S.M.A.S.M",
            "S.S.A.M.M",
            "M.S.A.M.S"
        ];

        for r in 0..self.rows - 2 {
            for c in 0..self.columns - 2 {
                let mut to_check = String::new();
                for i in 0..3 {
                    for j in 0..3 {
                        to_check.push(*self.char_at(r + i, c + j).unwrap());
                    }
                }

                for f in filters.iter() {
                    let found = to_check
                        .chars()
                        .enumerate()
                        .all(|(i, c)| {
                            let fc = f.chars().nth(i).unwrap();
                            fc == '.' || fc== c
                        });
                    if found {
                        matches += 1;
                    }
                }
            }
        }

        return matches;
    }

    fn match_word(&self, word_slice: &str, row: usize, col: usize, r_step: isize, c_step: isize) -> bool {
        let c: Option<&char> = self.char_at(row, col);
        return match c {
            Some(c) => {
                if !(*c == word_slice.chars().next().unwrap()) {
                    return false;
                }

                if word_slice.len() == 1 {
                    return true;
                }

                let next_row = row.checked_add_signed(r_step);
                let next_col = col.checked_add_signed(c_step);

                if next_row.is_none() || next_col.is_none() {
                    return false;
                }

                return self.match_word(&word_slice[1..], next_row.unwrap(), next_col.unwrap(), r_step, c_step)
            },
            None => false
        };
    }
}

fn init_puzzle(puzzle: &str) -> Puzzle {
    let rows = puzzle.lines().count();
    let columns = puzzle.lines().next().unwrap().len();
    let mut p = Vec::new();
    puzzle.chars().filter(|c| !c.is_whitespace() ).for_each(|c| p.push(c));

    return Puzzle { rows, columns, puzzle: p };
}

fn main() {

    let input = std::fs::read_to_string("input.txt").expect("Error reading input.txt");
    let puzzle = init_puzzle(&input);

    let part_1 = puzzle.find_words(&"XMAS");
    let part_2 = puzzle.find_x_mas();

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_1: &'static str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn given_input_expect_puzzle_created() {
        let p: Puzzle = init_puzzle(INPUT_1);
        assert_eq!("MMMSXXMASMMSAMXMSMSAAMXSXMAAMMMSAMASMSMXXMASAMXAMMXXAMMXXAMASMSMSASXSSSAXAMASAAAMAMMMXMMMMMXMXAXMASX", p.puzzle.iter().collect::<String>());
        assert_eq!(p.rows, 10);
        assert_eq!(p.columns, 10);
        assert_eq!(*p.char_at(0,0).unwrap(), 'M');
        assert_eq!(*p.char_at(4,7).unwrap(), 'A');
        assert_eq!(*p.char_at(9,9).unwrap(), 'X');
        assert!(p.char_at(10, 10).is_none());
    }

    #[test]
    fn given_input_expect_match_word_given() {
        let p: Puzzle = init_puzzle(INPUT_1);
        assert!(p.match_word(&"XMAS", 1, 4, 0, -1));
        assert!(p.match_word(&"XMAS", 5, 6, -1, -1));

        assert!(!p.match_word(&"XMAS", 9, 1, 0, -1));
    }

    #[test]
    fn given_input_find_all_words() {
        let p: Puzzle = init_puzzle(INPUT_1);
        assert_eq!(p.find_words(&"XMAS"), 18);
    }

    #[test]
    fn given_input_find_x_mas() {
        let p: Puzzle = init_puzzle(INPUT_1);
        assert_eq!(p.find_x_mas(), 9);
    }

}