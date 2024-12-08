use itertools::Itertools;

pub struct Line {
    result: i64,
    values: Vec<i64>,
}

#[derive(Clone, Copy, Debug)]
enum Operation {
    Add,
    Multiply,
    Concat,
}

#[aoc_generator(day7)]
pub fn generator(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|l| {
            let mut nums = l
                .replace(": ", " ")
                .split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect_vec();
            Line {
                result: nums.remove(0),
                values: nums,
            }
        })
        .collect_vec()
}

#[aoc(day7, part1)]
pub fn part1(input: &[Line]) -> i64 {
    input
        .iter()
        .filter(|l| {
            std::iter::repeat([Operation::Add, Operation::Multiply])
                .take(l.values.len() - 1)
                .multi_cartesian_product()
                .any(|assign| solve(l, &assign) == l.result)
        })
        .map(|l| l.result)
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &[Line]) -> i64 {
    input
        .iter()
        .filter(|l| {
            std::iter::repeat([Operation::Add, Operation::Multiply, Operation::Concat])
                .take(l.values.len() - 1)
                .multi_cartesian_product()
                .any(|assign| solve(l, &assign) == l.result)
        })
        .map(|l| l.result)
        .sum()
}

fn solve(line: &Line, ops: &[Operation]) -> i64 {
    let mut res = 0;
    for (i, num) in line.values.iter().enumerate() {
        if i == 0 {
            res = *num;
            continue;
        }
        match ops.get(i - 1) {
            Some(Operation::Add) => res += num,
            Some(Operation::Multiply) => res *= num,
            Some(Operation::Concat) => res = res * 10i64.pow(num.ilog10() + 1) + num,
            None => {}
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;
        assert_eq!(part1(&generator(input)), 3749);
    }

    #[test]
    fn part2_example() {
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;
        assert_eq!(part2(&generator(input)), 11387);
    }
}
