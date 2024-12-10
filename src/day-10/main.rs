#![feature(test)]
extern crate test;

use bstr::ByteSlice;
use fnv::FnvHashSet;
use itertools::Itertools;

const INPUT: &[u8] = include_bytes!("../../inputs/day-10.txt");

fn part_1(input: &[u8]) -> u64 {
    let map = input
        .lines()
        .map(|line| {
            line.iter()
                .map(|&c| match c {
                    b'0'..=b'9' => c - b'0',
                    // b'.' => 99,
                    _ => unsafe { std::hint::unreachable_unchecked() },
                })
                .collect_vec()
        })
        .collect_vec();

    let start_positions = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, &height)| (height == 0).then_some((y, x)))
        })
        .collect_vec();

    let mut cache = vec![vec![None; map[0].len()]; map.len()];

    start_positions
        .iter()
        .map(|&(y, x)| find_path_rating(&map, &mut cache, y, x).len() as u64)
        .sum()
}

fn find_path_rating(
    map: &Vec<Vec<u8>>,
    cache: &mut Vec<Vec<Option<FnvHashSet<(usize, usize)>>>>,
    y: usize,
    x: usize,
) -> FnvHashSet<(usize, usize)> {
    if let Some(value) = &cache[y][x] {
        return value.clone();
    }

    let curr_value = map[y][x];

    let reachable = [(0, 1), (1, 0), (0, -1), (-1, 0)]
        .into_iter()
        .filter_map(|(dy, dx)| {
            let (ny, nx) = (y as i32 + dy, x as i32 + dx);

            if ny >= 0 && ny < map.len() as i32 && nx >= 0 && nx < map[0].len() as i32 {
                let next_value = map[ny as usize][nx as usize];
                if next_value == 9 && next_value == curr_value + 1 {
                    let mut set = FnvHashSet::default();
                    set.insert((ny as usize, nx as usize));
                    return Some(set);
                } else if next_value == curr_value + 1 {
                    return Some(find_path_rating(
                        map,
                        cache,
                        ny as usize,
                        nx as usize,
                    ));
                };
            }
            None
        })
        .fold(FnvHashSet::default(), |acc, set| {
            acc.union(&set).copied().collect()
        });

    cache[y][x] = Some(reachable.clone());
    reachable
}

fn part_2(input: &[u8]) -> u64 {
    let map = input
        .lines()
        .map(|line| {
            line.iter()
                .map(|&c| match c {
                    b'0'..=b'9' => c - b'0',
                    // b'.' => 99,
                    _ => unsafe { std::hint::unreachable_unchecked() },
                })
                .collect_vec()
        })
        .collect_vec();

    let start_positions = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, &height)| (height == 0).then_some((y, x)))
        })
        .collect_vec();

    let mut cache = vec![vec![None; map[0].len()]; map.len()];

    start_positions
        .iter()
        .map(|&(y, x)| find_path_rating2(&map, &mut cache, y, x) as u64)
        .sum()
}

fn find_path_rating2(
    map: &Vec<Vec<u8>>,
    cache: &mut Vec<Vec<Option<u64>>>,
    y: usize,
    x: usize,
) -> u64 {
    if let Some(value) = cache[y][x] {
        return value;
    }

    let curr_value = map[y][x];

    let reachable = [(0, 1), (1, 0), (0, -1), (-1, 0)]
        .into_iter()
        .filter_map(|(dy, dx)| {
            let (ny, nx) = (y as i32 + dy, x as i32 + dx);

            if ny >= 0 && ny < map.len() as i32 && nx >= 0 && nx < map[0].len() as i32 {
                let next_value = map[ny as usize][nx as usize];
                if next_value == 9 && next_value == curr_value + 1 {
                    return Some(1);
                } else if next_value == curr_value + 1 {
                    return Some(find_path_rating2(
                        map,
                        cache,
                        ny as usize,
                        nx as usize,
                    ));
                };
            }
            None
        })
        .sum();
    cache[y][x] = Some(reachable);
    reachable
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;

    const EXAMPLE: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE.as_bytes()), 36);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE.as_bytes()), 81);
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
