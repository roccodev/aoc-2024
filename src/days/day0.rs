use itertools::Itertools;

#[aoc_generator(day0)]
pub fn generator(input: &str) -> Input {
    0
}

#[aoc(day0, part1)]
pub fn part1(input: &Input) -> u32 {
    0
}

#[aoc(day0, part2)]
pub fn part2(input: &Input) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#""#;
        assert_eq!(part1(&generator(input)), 0);
    }

    #[test]
    fn part2_example() {
        let input = r#""#;
        assert_eq!(part2(&generator(input)), 0);
    }
}
