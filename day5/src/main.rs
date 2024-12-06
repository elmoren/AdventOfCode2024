use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

// page number key must come before all pages in HashSet
#[derive(Debug)]
struct PrintRules {
    ordering_rules: HashMap<i32, HashSet<i32>>,
    pages: Vec<String>
}

impl PrintRules {
    fn find_correctly_ordered(&self) -> Vec<&str> {
        return self.pages
            .iter()
            .map(|p| p.as_str())
            .filter(|l| self.check_ordering(l))
            .collect();
    }

    fn score_part_1(&self) -> u32 {
        self
            .find_correctly_ordered()
            .into_iter()
            .map(|p: &str| {
                let v: Vec<i32> = p
                    .split(",")
                    .map(|s| s.parse::<i32>().unwrap_or(0))
                    .collect::<Vec<i32>>();

                return v[v.len().div_euclid(2)] as u32;
            })
            .sum::<u32>()
    }

    fn check_ordering(&self, l: &str) -> bool {
        let page_nums = l.split(",")
            .map(|s| s.parse::<i32>().unwrap_or(0))
            .collect::<Vec<i32>>();

        return page_nums.is_sorted_by(|a, b| self.page_cmp(a, b).is_lt());
    }

    fn find_incorrectly_ordered(&self) -> Vec<&str> {
        return self.pages
            .iter()
            .map(|p| p.as_str())
            .filter(|l| !self.check_ordering(l))
            .collect();
    }

    fn page_cmp(&self, a: &i32, b: &i32) -> Ordering {
        let Some(a_rules) = self.ordering_rules.get(a) else {
            return Ordering::Greater;
        };

        let Some(b_rules) = self.ordering_rules.get(b) else {
            return Ordering::Less;
        };

        if a_rules.contains(b) {
            return Ordering::Less;
        } else if b_rules.contains(a) {
            return Ordering::Greater;
        } else {
            return Ordering::Equal;
        }
    }

    fn score_part_2(&self) -> u32 {
        self
            .find_incorrectly_ordered()
            .into_iter()
            .map(|p: &str| {
                let mut v: Vec<i32> = p
                    .split(",")
                    .map(|s| s.parse::<i32>().unwrap_or(0))
                    .collect::<Vec<i32>>();

                v.sort_by(|a, b| self.page_cmp(a, b));

                return v[v.len().div_euclid(2)] as u32;
            })
            .sum::<u32>()
    }
}

fn parse_input<'a>(input: &'a str) -> (Vec<&'a str>, Vec<&'a str>) {
    let mut input_iter = input.lines();

    let rules = input_iter
        .by_ref()
        .take_while(|l| l.len() > 0)
        .collect::<Vec<&str>>();

    let pages_input =
        input_iter.collect::<Vec<&str>>();
    (rules, pages_input)
}


fn init(rules: Vec<&str>, pages_input: Vec<&str>) -> PrintRules {
    let mut ordering_rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    rules
        .iter()
        .map(|s| s.split_once("|").unwrap())
        .map(|t| (t.0.parse::<i32>().unwrap(), t.1.parse::<i32>().unwrap() ))
        .for_each(|e| {
            let h = ordering_rules.entry(e.0).or_insert(HashSet::new());
            h.insert(e.1);
        });

    let pages = pages_input.iter().map(|s| String::from(*s)).collect();

    return PrintRules {ordering_rules, pages};
}

fn main() {

    let input = fs::read_to_string("input.txt").expect("File does not exist");
    let (rules, pages_input) = parse_input(&input);
    let print_rules = init(rules, pages_input);

    let part_1 = print_rules.score_part_1();
    let part_2 = print_rules.score_part_2();

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
        fn given_input_sum_reports() {
        let (rules, pages_input) = parse_input(SAMPLE);
        let print_rules = init(rules, pages_input);
        let result = print_rules.score_part_1();
        assert_eq!(result, 143);
    }

    #[test]
    fn given_input_fix_and_sum_reports() {
        let (rules, pages_input) = parse_input(SAMPLE);
        let print_rules = init(rules, pages_input);
        let result = print_rules.score_part_2();
        assert_eq!(result, 123);
    }
}
