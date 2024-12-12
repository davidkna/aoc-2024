#![feature(test)]
extern crate test;

use bstr::ByteSlice;
use itertools::Itertools;

const INPUT: &[u8] = include_bytes!("../../inputs/day-12.txt");

fn parse_input(input: &[u8]) -> (Vec<Vec<usize>>, usize) {
    let map = input
        .lines()
        .map(|line| line.iter().copied().collect_vec())
        .collect_vec();

    map.iter().enumerate().fold(
        (vec![vec![0; map[0].len()]; map.len()], 0usize),
        |(mut acc, mut count), (y, line)| {
            line.iter().enumerate().for_each(|(x, &c)| {
                let above = if y == 0 {
                    None
                } else {
                    map.get(y - 1).and_then(|line| line.get(x))
                };

                let left = if x == 0 {
                    None
                } else {
                    map.get(y).and_then(|line| line.get(x - 1))
                };

                if left == Some(&c) && above == Some(&c) && acc[y - 1][x] != acc[y][x - 1] {
                    let min = acc[y - 1][x].min(acc[y][x - 1]);
                    let max = acc[y - 1][x].max(acc[y][x - 1]);
                    acc[y][x] = min;
                    acc.iter_mut().for_each(|line| {
                        line.iter_mut().for_each(|cell| {
                            if *cell == max {
                                *cell = min;
                            }
                            if *cell == count - 1 {
                                *cell = max;
                            }
                        });
                    });
                    count -= 1;
                    return;
                }

                if above == Some(&c) {
                    acc[y][x] = acc[y - 1][x];
                    return;
                }

                if left == Some(&c) {
                    acc[y][x] = acc[y][x - 1];
                    return;
                }
                acc[y][x] = count;
                count += 1;
            });
            (acc, count)
        },
    )
}

fn part_1(input: &[u8]) -> u64 {
    let (map_uniq, count) = parse_input(input);

    let (areas, perimeters) = map_uniq.iter().enumerate().fold(
        (vec![0; count], vec![0; count]),
        |(mut areas, mut perimeters), (y, line)| {
            line.iter().enumerate().for_each(|(x, &c)| {
                let get = |y: i32, x: i32| {
                    let yu: usize = y.try_into().ok()?;
                    let xu: usize = x.try_into().ok()?;

                    map_uniq.get(yu).and_then(|line| line.get(xu))
                };

                let left = get(y as i32, x as i32 - 1);
                let above = get(y as i32 - 1, x as i32);

                areas[c] += 1;

                if left != Some(&c) {
                    perimeters[c] += 1;
                    if let Some(l) = left {
                        perimeters[*l] += 1;
                    }
                }

                if above != Some(&c) {
                    perimeters[c] += 1;
                    if let Some(a) = above {
                        perimeters[*a] += 1;
                    }
                }

                if y == map_uniq.len() - 1 {
                    perimeters[c] += 1;
                }

                if x == line.len() - 1 {
                    perimeters[c] += 1;
                }
            });
            (areas, perimeters)
        },
    );

    std::iter::zip(areas.iter(), perimeters.iter())
        .map(|(area, perimeter)| area * perimeter)
        .sum()
}

fn part_2(input: &[u8]) -> u64 {
    let (map_uniq, count) = parse_input(input);

    let (areas, perimeters) = map_uniq.iter().enumerate().fold(
        (vec![0; count], vec![0; count]),
        |(mut areas, mut perimeters), (y, line)| {
            line.iter().enumerate().for_each(|(x, &c)| {
                let get = |y: i32, x: i32| {
                    let yu: usize = y.try_into().ok()?;
                    let xu: usize = x.try_into().ok()?;

                    map_uniq.get(yu).and_then(|line| line.get(xu))
                };

                let left = get(y as i32, x as i32 - 1);
                let above = get(y as i32 - 1, x as i32);
                let right = get(y as i32, x as i32 + 1);
                let down = get(y as i32 + 1, x as i32);

                areas[c] += 1;

                // Check left perimeter
                if left != Some(&c) {
                    // Skip if above also has left perimeter
                    if above != Some(&c) || get(y as i32 - 1, x as i32 - 1) == Some(&c) {
                        perimeters[c] += 1;
                    }
                }

                // Check above perimeter
                if above != Some(&c) {
                    // Skip if left also has above perimeter
                    if left != Some(&c) || get(y as i32 - 1, x as i32 - 1) == Some(&c) {
                        perimeters[c] += 1;
                    }
                }

                // Check right perimeter
                if right != Some(&c) {
                    // Skip if above also has right perimeter
                    if above != Some(&c) || get(y as i32 - 1, x as i32 + 1) == Some(&c) {
                        perimeters[c] += 1;
                    }
                }

                // Check down perimeter
                if down != Some(&c) {
                    // Skip if left also has down perimeter
                    if left != Some(&c) || get(y as i32 + 1, x as i32 - 1) == Some(&c) {
                        perimeters[c] += 1;
                    }
                }
            });
            (areas, perimeters)
        },
    );

    std::iter::zip(areas.iter(), perimeters.iter())
        .map(|(area, perimeter)| area * perimeter)
        .sum()
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;

    const EXAMPLE_A: &str = "AAAA
BBCD
BBCC
EEEC";

    const EXAMPLE_B: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    const EXAMPLE_E: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE_A.as_bytes()), 140);
        assert_eq!(part_1(EXAMPLE_B.as_bytes()), 1930);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE_A.as_bytes()), 80);
        assert_eq!(part_2(EXAMPLE_E.as_bytes()), 236);
    }

    #[bench]
    fn bench_part_1(b: &mut test::Bencher) {
        b.iter(|| part_1(black_box(INPUT)));
    }

    #[bench]
    fn bench_part_2(b: &mut test::Bencher) {
        b.iter(|| part_2(black_box(INPUT)));
    }
}
