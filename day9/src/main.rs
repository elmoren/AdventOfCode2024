use std::cmp::min;
use std::collections::VecDeque;

#[derive(Debug)]
struct FileNode {
    id: u64,
    parts: VecDeque<(u32, u32)>, // start, length
}

#[derive(Debug)]
struct DiskMap {
    files: Vec<FileNode>,
    free_pages: VecDeque<(u32, u32)>, // start, length
}

// Iterate over all files in reverse, if file starts after the earliest free page, then we process.
// Assumption: files and free pages are sorted by start index
// Assumption: input is clean with no fragmentation existing
fn fragment(dm: &mut DiskMap) {
    for f in &mut dm.files.iter_mut().rev() {
        let mut part = f.parts.pop_front().unwrap();
        while part.1 > 0 {
            let mut free_page = dm.free_pages.pop_front().unwrap();
            if free_page.1 == 0 {
                continue;
            }

            if free_page.0 >= part.0 {
                f.parts.push_back(part);
                dm.free_pages.push_front(free_page);
                break;
            }

            let move_length = min(part.1, free_page.1);
            let file_part: (u32, u32) = (free_page.0, move_length);
            f.parts.push_back(file_part);

            // Keep track of the amount moved
            part.1 -= move_length;
            free_page.0 += move_length;
            free_page.1 -= move_length;

            // Add moved part back to free_pages

            dm.free_pages.push_front(free_page);
        }
    }
}

fn defragment_whole_files(dm: &mut DiskMap) {
    dm.files.sort_by_key(|f| f.id);
    for f in &mut dm.files.iter_mut().rev() {
        println!("File Id: {:?}", f.id);
        let mut part: &mut (u32, u32) = f.parts.get_mut(0).unwrap();
        if part.1 == 0 {
            continue;
        }

        for free_page in &mut dm.free_pages.iter_mut() {
            if free_page.1 >= part.1 && free_page.0 < part.0 {
                let file_part: (u32, u32) = (free_page.0, part.1);

                part.0 = free_page.0;
                free_page.0 += part.1;
                free_page.1 -= part.1;

                break;
            }
        }
    }
}

fn checksum(disk: &DiskMap) -> u64 {
    disk.files.iter().map(|f| checksum_file(&f)).sum()
}

fn checksum_file(f: &FileNode) -> u64 {
    f.parts
        .iter()
        .map(|p| (p.0..p.0 + p.1).fold(0, |acc, n| acc + (f.id * n as u64)))
        .sum()
}

fn init_map(input: &str) -> DiskMap {
    let mut files: Vec<FileNode> = Vec::new();
    let mut free_pages: VecDeque<(u32, u32)> = VecDeque::new();
    let mut is_file = true;
    let mut offset = 0;
    let mut id = 0;

    for c in input.chars() {
        let val: u32 = c.to_digit(10).unwrap();
        if is_file {
            files.push(FileNode {
                id,
                parts: VecDeque::from(vec![(offset, val)]),
            });
            id += 1;
        } else if val > 0 {
            free_pages.push_back((offset, val));
        }

        offset += val;
        is_file = !is_file;
    }

    DiskMap { files, free_pages }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("cannot read file");
    let mut map = init_map(&input);
    // fragment(&mut map);
    defragment_whole_files(&mut map);
    println!("Result: {}", checksum(&map));
}

#[cfg(test)]
mod test {
    use super::*;

    // const INPUT: &str = "12345";
    const INPUT: &str = "2333133121414131402";

    // 00
    // 99
    // 811188827773336446555566

    #[test]
    fn test_part_1() {
        let mut dm = init_map(INPUT);
        fragment(&mut dm);
        assert_eq!(checksum(&dm), 1928);
    }

    #[test]
    fn test_part_2() {
        let mut dm = init_map(INPUT);
        defragment_whole_files(&mut dm);
        assert_eq!(checksum(&dm), 2858);
    }

    #[test]
    fn validate_checksum() {
        let dm = DiskMap {
            files: vec![
                FileNode {
                    id: 0,
                    parts: vec![(0, 1)].into(),
                },
                FileNode {
                    id: 9,
                    parts: vec![(2, 2)].into(),
                },
                FileNode {
                    id: 8,
                    parts: vec![(4, 1), (8, 3)].into(),
                },
            ],
            free_pages: vec![].into(),
        };

        assert_eq!(checksum_file(&dm.files[0]), 0);
        assert_eq!(checksum_file(&dm.files[1]), 45);
        assert_eq!(checksum_file(&dm.files[2]), 248);
        assert_eq!(checksum(&dm), 293);
    }
}
