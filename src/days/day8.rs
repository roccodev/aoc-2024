use fxhash::{FxHashMap, FxHashSet};
use itertools::{FoldWhile, Itertools};

pub struct Input {
    frequencies: FxHashMap<u8, Vec<(isize, isize)>>,
    max: (isize, isize),
}

#[aoc_generator(day8)]
pub fn generator(input: &str) -> Input {
    let mut frequencies: FxHashMap<u8, Vec<(isize, isize)>> = FxHashMap::default();
    let mut max = (0, 0);
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.bytes().enumerate() {
            if c != b'.' {
                frequencies
                    .entry(c)
                    .or_default()
                    .push((x as isize, y as isize));
            }
            max.0 = x as isize;
        }
        max.1 = y as isize;
    }
    Input { max, frequencies }
}

#[aoc(day8, part1)]
pub fn part1(input: &Input) -> usize {
    let mut possible = FxHashSet::default();
    for (_, locs) in &input.frequencies {
        possible.extend(
            locs.iter()
                .permutations(2)
                .filter_map(|perm| {
                    let [a, b] = perm.as_slice() else {
                        unreachable!()
                    };
                    if a == b {
                        return None;
                    }
                    let diff = (b.0 - a.0, b.1 - a.1);
                    Some(((a.0 - diff.0, a.1 - diff.1), (b.0 + diff.0, b.1 + diff.1)))
                })
                .flat_map(|(a, b)| [a, b])
                .filter(|loc| {
                    loc.0 >= 0 && loc.0 <= input.max.0 && loc.1 >= 0 && loc.1 <= input.max.1
                }),
        );
    }
    possible.len()
}

#[aoc(day8, part2)]
pub fn part2(input: &Input) -> usize {
    let mut possible = FxHashSet::default();
    for (_, locs) in &input.frequencies {
        possible.extend(
            locs.iter()
                .permutations(2)
                .filter_map(|perm| {
                    let [a, b] = perm.as_slice() else {
                        unreachable!()
                    };
                    if a == b {
                        return None;
                    }
                    let diff = ((b.0 - a.0), (b.1 - a.1));
                    Some((Some(*a), Some(*b), diff))
                })
                .flat_map(|(a, b, diff)| {
                    std::iter::once((a, b, diff))
                        .cycle()
                        .fold_while(
                            (vec![], a.copied(), b.copied(), diff),
                            |(mut list, a, b, diff), _| {
                                let mut next_a = None;
                                let mut next_b = None;
                                if let Some(a) = a {
                                    list.push(a);
                                    next_a = Some((a.0 - diff.0, a.1 - diff.1)).filter(|loc| {
                                        loc.0 >= 0
                                            && loc.0 <= input.max.0
                                            && loc.1 >= 0
                                            && loc.1 <= input.max.1
                                    });
                                }
                                if let Some(b) = b {
                                    list.push(b);
                                    next_b = Some((b.0 + diff.0, b.1 + diff.1)).filter(|loc| {
                                        loc.0 >= 0
                                            && loc.0 <= input.max.0
                                            && loc.1 >= 0
                                            && loc.1 <= input.max.1
                                    });
                                }
                                if next_a.is_none() && next_b.is_none() {
                                    FoldWhile::Done((list, next_a, next_b, diff))
                                } else {
                                    FoldWhile::Continue((list, next_a, next_b, diff))
                                }
                            },
                        )
                        .into_inner()
                        .0
                }),
        );
    }
    possible.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;
        assert_eq!(part1(&generator(input)), 14);
    }

    #[test]
    fn part2_example() {
        let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;
        assert_eq!(part2(&generator(input)), 34);
    }
}
