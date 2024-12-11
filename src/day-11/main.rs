#![feature(test)]
extern crate test;

use bstr::ByteSlice;
use fnv::FnvHashMap;
use itertools::Itertools;

const INPUT: &[u8] = include_bytes!("../../inputs/day-11.txt"); // xxx

fn parse_uint(s: &[u8]) -> u64 {
    s.iter().fold(0, |acc, &c| acc * 10 + (c - b'0') as u64)
}

fn solve(input: &[u8], iters: usize) -> u64 {
    let stones = input.split_str(" ").map(parse_uint).collect_vec();

    let mut stone_map: FnvHashMap<u64, u64> =
        stones
            .iter()
            .fold(FnvHashMap::default(), |mut acc, &stone| {
                *acc.entry(stone).or_default() += 1;
                acc
            });

    for _ in 0..iters {
        stone_map = stone_map
            .iter()
            .fold(FnvHashMap::default(), |mut acc, (&stone, &count)| {
                if stone == 0 {
                    *acc.entry(1).or_default() += count;
                    return acc;
                }

                let digits = stone.ilog10() + 1;

                if digits % 2 == 0 {
                    let left_half = stone / (10u64.pow(digits / 2));
                    let right_half = stone % (10u64.pow(digits / 2));

                    *acc.entry(left_half).or_default() += count;
                    *acc.entry(right_half).or_default() += count;
                    return acc;
                }
                *acc.entry(2024 * stone).or_default() += count;
                acc
            });
    }
    stone_map.values().sum()
}

fn part_1(input: &[u8]) -> u64 {
    solve(input, 25)
}

fn part_2(input: &[u8]) -> u64 {
    solve(input, 75)
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;

    const EXAMPLE: &str = "125 17";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE.as_bytes()), 55312);
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
