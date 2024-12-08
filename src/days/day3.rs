use regex::Regex;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    let re = Regex::new(r#"mul\((\d+),(\d+)\)"#).unwrap();
    re.captures_iter(input)
        .map(|caps| {
            let a: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
            let b: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
            a * b
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> i32 {
    let re = Regex::new(r#"(don't\(\)|do\(\)|mul\((\d+),(\d+)\))"#).unwrap();
    let mut operate = true;
    re.captures_iter(input)
        .map(|caps| match caps.get(2) {
            Some(a) => {
                let a: i32 = a.as_str().parse().unwrap();
                let b: i32 = caps.get(3).unwrap().as_str().parse().unwrap();
                if operate {
                    a * b
                } else {
                    0
                }
            }
            None => {
                operate = caps.get(1).unwrap().as_str() == "do()";
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
        assert_eq!(part1(&input), 161);
    }

    #[test]
    fn part2_example() {
        let input = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;
        assert_eq!(part2(&input), 48);
    }
}
