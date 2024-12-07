#![feature(test)]
extern crate test;

use bstr::ByteSlice;
use itertools::Itertools;
use rayon::prelude::*;

const INPUT: &[u8] = include_bytes!("../../inputs/day-07.txt");

enum Operation {
    Add,
    Mul,
    Concat,
}

impl Operation {
    #[inline]
    fn execute(&self, a: u64, b: u64) -> u64 {
        match self {
            Operation::Add => a + b,
            Operation::Mul => a * b,
            Operation::Concat => {
                let mut multiplier = 1;
                let mut n = b;

                while n > 0 {
                    multiplier *= 10;
                    n /= 10;
                }

                a * multiplier + b
            }
        }
    }
}

fn parse_uint(s: &[u8]) -> u64 {
    s.iter().fold(0, |acc, &c| acc * 10 + (c - b'0') as u64)
}

fn part_1(input: &[u8]) -> u64 {
    solve(input, &[Operation::Add, Operation::Mul])
}

fn part_2(input: &[u8]) -> u64 {
    solve(input, &[Operation::Add, Operation::Mul, Operation::Concat])
}

fn solve(input: &[u8], ops: &[Operation]) -> u64 {
    input
        .lines()
        .collect_vec()
        .into_par_iter()
        .filter_map(|line| {
            let (target, numbers) = line.split_once_str(": ").unwrap();
            let target = parse_uint(target);
            let numbers = numbers.split_str(" ").map(parse_uint).collect_vec();

            let mut stack = vec![(1, numbers[0])];

            while let Some((idx, result)) = stack.pop() {
                if idx == numbers.len() {
                    if result == target {
                        return Some(target);
                    }
                    continue;
                }

                for op in ops {
                    let r = op.execute(result, numbers[idx]);
                    if r <= target {
                        stack.push((idx + 1, r));
                    }
                }
            }

            None
        })
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

    const EXAMPLE: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE.as_bytes()), 3749);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE.as_bytes()), 11387);
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
