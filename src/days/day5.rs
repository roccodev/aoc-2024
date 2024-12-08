use std::cmp::Ordering;

use fxhash::FxHashSet;
use itertools::Itertools;

pub struct Input {
    rules: FxHashSet<(i32, i32)>,
    pages: Vec<Vec<i32>>,
}

#[aoc_generator(day5)]
pub fn generator(input: &str) -> Input {
    let (rules, pages) = input.split("\n\n").collect_tuple().unwrap();
    Input {
        rules: rules
            .lines()
            .map(|l| {
                l.split("|")
                    .map(|s| s.parse().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect(),
        pages: pages
            .lines()
            .map(|l| l.split(",").map(|s| s.parse().unwrap()).collect_vec())
            .collect_vec(),
    }
}

#[aoc(day5, part1)]
pub fn part1(input: &Input) -> i32 {
    input
        .pages
        .iter()
        .filter(|page| page.is_sorted_by(|a, b| !input.rules.contains(&(*b, *a))))
        .map(|page| page[page.len() / 2])
        .sum()
}

#[aoc(day5, part2)]
pub fn part2(input: &Input) -> i32 {
    let mut pages = input.pages.clone();
    let mut total = 0;
    for page in &mut pages {
        if page.is_sorted_by(|a, b| !input.rules.contains(&(*b, *a))) {
            continue;
        }
        page.sort_by(|&a, &b| {
            if input.rules.contains(&(a, b)) {
                Ordering::Less
            } else if input.rules.contains(&(b, a)) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
        total += page[page.len() / 2];
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"47|53
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
97,13,75,29,47"#;
        assert_eq!(part1(&generator(input)), 143);
    }

    #[test]
    fn part2_example() {
        let input = r#"47|53
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
97,13,75,29,47"#;
        assert_eq!(part2(&generator(input)), 123);
    }
}
