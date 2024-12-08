use itertools::Itertools;

#[aoc_generator(day4)]
pub fn generator(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|s| s.bytes().collect_vec()).collect()
}

#[aoc(day4, part1)]
pub fn part1(input: &[Vec<u8>]) -> u32 {
    let mut total = 0;
    for (y, line) in input.iter().enumerate() {
        for (x, &ch) in line.iter().enumerate() {
            // Check in all directions from each S
            if ch == b'S' {
                for dir in [
                    (-1, -1),
                    (1, 0),
                    (0, 1),
                    (-1, 0),
                    (0, -1),
                    (1, 1),
                    (1, -1),
                    (-1, 1),
                ] {
                    if cast_dir(input, (x as isize, y as isize), dir) {
                        total += 1
                    }
                }
            }
        }
    }
    total
}

#[aoc(day4, part2)]
pub fn part2(input: &[Vec<u8>]) -> i32 {
    let mut total = 0;
    for (y, line) in input.iter().enumerate() {
        for (x, &ch) in line.iter().enumerate() {
            // Check corners of each A
            if ch == b'A' {
                let top = y.checked_sub(1);
                let left = x.checked_sub(1);
                let (tl, tr, bl, br) = (
                    top.and_then(|y| left.map(|x| input[y][x])),
                    top.and_then(|y| input[y].get(x + 1)),
                    input.get(y + 1).and_then(|l| left.and_then(|x| l.get(x))),
                    input.get(y + 1).and_then(|l| l.get(x + 1)),
                );
                let (Some(tl), Some(&tr), Some(&bl), Some(&br)) = (tl, tr, bl, br) else {
                    continue;
                };
                if [tl, tr, bl, br].iter().any(|&c| c == b'A' || c == b'X') {
                    continue;
                }
                if br != tl && bl != tr {
                    total += 1;
                }
            }
        }
    }
    total
}

fn cast_dir(input: &[Vec<u8>], mut pos: (isize, isize), dir: (isize, isize)) -> bool {
    for exp in [b'A', b'M', b'X'] {
        pos.0 += dir.0;
        pos.1 += dir.1;
        match (usize::try_from(pos.0), usize::try_from(pos.1)) {
            (Ok(x), Ok(y)) => {
                if input
                    .get(y)
                    .and_then(|l| l.get(x))
                    .is_none_or(|v| *v != exp)
                {
                    return false;
                }
            }
            _ => return false,
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        assert_eq!(part1(&generator(input)), 18);
    }

    #[test]
    fn part2_example() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        assert_eq!(part2(&generator(input)), 9);
    }
}
