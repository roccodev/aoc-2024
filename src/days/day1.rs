use itertools::Itertools;
use std::str::FromStr;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let (mut first, mut second): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|s| {
            let (f, s) = s
                .split("   ")
                .map(|s| i32::from_str(s).unwrap())
                .collect_tuple()
                .unwrap();
            (f, s)
        })
        .unzip();
    first.sort_unstable();
    second.sort_unstable();
    first
        .into_iter()
        .zip(second.into_iter())
        .map(|(a, b)| a.abs_diff(b))
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let (first, mut second): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|s| {
            let (f, s) = s
                .split("   ")
                .map(|s| i32::from_str(s).unwrap())
                .collect_tuple()
                .unwrap();
            (f, s)
        })
        .unzip();

    let mut counts: Vec<(i32, i32)> = Vec::new();

    second.sort_unstable();

    let mut last = 0;
    for n in second {
        if n == last {
            let l = counts.len() - 1;
            counts[l].1 += 1;
        } else {
            counts.push((n, 1));
        }
        last = n;
    }

    first
        .into_iter()
        .map(|n| {
            let count = counts
                .binary_search_by_key(&n, |&(a, _)| a)
                .map(|i| counts[i].1)
                .unwrap_or_default();
            count * n
        })
        .sum()
}

// This was my first time doing AoC at 6 AM, can't say it was worth it.
// Here are the solutions I wrote then (492/243, 02:24/03:18)

#[allow(unused)]
fn part1_6am(input: &str) -> u32 {
    let mut first: Vec<i32> = input
        .lines()
        .map(|s| s.split_ascii_whitespace().next().unwrap().parse().unwrap())
        .collect_vec();
    let mut second: Vec<i32> = input
        .lines()
        .map(|s| s.split_ascii_whitespace().nth(1).unwrap().parse().unwrap())
        .collect_vec();
    first.sort();
    second.sort();
    let mut tot = 0;
    for i in 0..first.len() {
        tot += first[i].abs_diff(second[i]);
    }
    tot
}

#[allow(unused)]
fn part2_6am(input: &str) -> usize {
    let mut first: Vec<i32> = input
        .lines()
        .map(|s| s.split_ascii_whitespace().next().unwrap().parse().unwrap())
        .collect_vec();
    let mut second: Vec<i32> = input
        .lines()
        .map(|s| s.split_ascii_whitespace().nth(1).unwrap().parse().unwrap())
        .collect_vec();
    first.sort();
    second.sort();
    let mut tot = 0;
    for i in 0..first.len() {
        tot += first[i] as usize * second.iter().filter(|x| **x == first[i]).count()
    }
    tot
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
        assert_eq!(part1(input), 11);
        assert_eq!(part1_6am(input), part1(input));
    }

    #[test]
    fn part2_example() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
        assert_eq!(part2(input), 31);
        assert_eq!(part2_6am(input) as i32, part2(input));
    }
}
