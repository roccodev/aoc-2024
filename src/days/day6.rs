use fxhash::FxHashSet;
use itertools::Itertools;

pub struct Input {
    pos: (isize, isize),
    size: (isize, isize),
    obstructions: FxHashSet<(isize, isize)>,
}

#[aoc_generator(day6)]
pub fn generator(input: &str) -> Input {
    let mut pos = (0, 0);
    let mut obstructions = FxHashSet::default();
    let mut size = (0, 0);
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c == '^' {
                pos = (x as isize, y as isize);
            } else if c == '#' {
                obstructions.insert((x as isize, y as isize));
            }
            size.0 = x as isize;
        }
        size.1 = y as isize;
    }
    Input {
        pos,
        size,
        obstructions,
    }
}

#[aoc(day6, part1)]
pub fn part1(input: &Input) -> usize {
    visit_pos(input).len()
}

#[aoc(day6, part2)]
pub fn part2(input: &Input) -> usize {
    let visited = visit_pos(input);
    let mut possible = FxHashSet::default();
    let mut obstructions = input.obstructions.clone();

    for pos in visited {
        if pos == input.pos {
            continue;
        }
        obstructions.insert(pos);
        if run_until_loop_or_exit(&input, &obstructions, input.pos, 0) {
            possible.insert(pos);
        }
        obstructions.remove(&pos);
    }

    possible.len()
}

fn visit_pos(input: &Input) -> FxHashSet<(isize, isize)> {
    let mut visited = FxHashSet::default();
    let mut pos = input.pos;
    let dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut dir_idx = 0usize;

    loop {
        visited.insert(pos);
        let dir = dirs[dir_idx];
        if input.obstructions.contains(&(pos.0 + dir.0, pos.1 + dir.1)) {
            dir_idx = (dir_idx + 1) % dirs.len();
        }
        let dir = dirs[dir_idx];
        pos.0 += dir.0;
        pos.1 += dir.1;
        if pos.0 < 0 || pos.1 < 0 || pos.0 > input.size.0 || pos.1 > input.size.1 {
            break;
        }
    }

    visited
}

fn run_until_loop_or_exit(
    input: &Input,
    obstructions: &FxHashSet<(isize, isize)>,
    mut pos: (isize, isize),
    mut dir_idx: usize,
) -> bool {
    let mut visited = FxHashSet::default();
    let dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    loop {
        let mut dir = dirs[dir_idx];
        let mut obs = (pos.0 + dir.0, pos.1 + dir.1);
        while obstructions.contains(&obs) {
            if visited.contains(&(obs, dir_idx)) {
                // Found loop
                return true;
            }
            visited.insert((obs, dir_idx));
            dir_idx = (dir_idx + 1) % dirs.len();
            dir = dirs[dir_idx];
            obs = (pos.0 + dir.0, pos.1 + dir.1);
        }
        pos.0 += dir.0;
        pos.1 += dir.1;
        if pos.0 < 0 || pos.1 < 0 || pos.0 > input.size.0 || pos.1 > input.size.1 {
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
        assert_eq!(part1(&generator(input)), 41);
    }

    #[test]
    fn part2_example() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
        assert_eq!(part2(&generator(input)), 6);
    }
}
