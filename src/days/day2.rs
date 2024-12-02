use itertools::Itertools;

#[derive(Clone)]
pub struct Line {
    levels: Vec<i32>,
}

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|l| Line {
            levels: l
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Line]) -> usize {
    input
        .iter()
        .filter(|l| {
            l.levels
                .iter()
                .tuple_windows()
                .all(|(a, b, c)| is_valid(*a, *b, *c))
        })
        .count()
}

#[aoc(day2, part2)]
pub fn part2(input: &[Line]) -> usize {
    let mut input = input.to_vec();
    let mut valid = 0;
    for l in &mut input {
        let cond = |l: &mut Line| {
            let mut fixed = false;
            let mut i = 0;
            loop {
                if i + 2 >= l.levels.len() {
                    return true;
                }
                let (a, b, c) = (l.levels[i], l.levels[i + 1], l.levels[i + 2]);
                if is_valid(a, b, c) {
                    i += 1;
                    continue;
                }
                if fixed {
                    return false;
                }
                if i + 3 >= l.levels.len() {
                    return true;
                }
                fixed = true;
                let d = l.levels[i + 3];
                // Remove a
                if is_valid(b, c, d) {
                    i += 1;
                    continue;
                }
                // Remove b, sgn(b-a) is the ordering thus far
                if is_valid(a, c, d) && (c - a).signum() == (b - a).signum() {
                    l.levels[i + 1] = a;
                    i += 1;
                    continue;
                }
                // Remove c
                if is_valid(a, b, d) {
                    l.levels[i + 1] = a;
                    l.levels[i + 2] = b;
                    i += 1;
                    continue;
                }
                return false;
            }
        };
        if cond(l) {
            valid += 1;
        }
    }
    valid
}

fn is_valid(a: i32, b: i32, c: i32) -> bool {
    let d1 = b - a;
    let d2 = c - b;
    d1.signum() == d2.signum() && (1..=3).contains(&d1.abs()) && (1..=3).contains(&d2.abs())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
        assert_eq!(part1(&generator(input)), 2);
    }

    #[test]
    fn part2_example() {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
        assert_eq!(part2(&generator(input)), 4);
    }
}
