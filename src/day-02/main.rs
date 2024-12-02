#![feature(test)]
extern crate test;

use std::cmp::Ordering;

use bstr::ByteSlice;
use itertools::Itertools;

const INPUT: &[u8] = include_bytes!("../../inputs/day-02.txt");

fn part_1(input: &[u8]) -> u32 {
    input
        .lines()
        .filter(|line| {
            let mut iter = line
                .split_str(" ")
                .map(|num| unsafe { num.to_str_unchecked() }.parse::<u32>().unwrap())
                .tuple_windows();

            let (a, b) = iter.next().unwrap();
            let trend = a.cmp(&b);
            if trend == Ordering::Equal || a.abs_diff(b) > 3 {
                return false;
            }

            iter.all(|(a, b)| a.cmp(&b) == trend && a.abs_diff(b) <= 3)
        })
        .count() as u32
}
fn part_2(input: &[u8]) -> u32 {
    input
        .lines()
        .filter(|line| {
            let mut row = line
                .split_str(" ")
                .map(|num| unsafe { num.to_str_unchecked() }.parse::<u32>().unwrap())
                .collect_vec();

            let check_row = |data: &[u32], order: Ordering| {
                let mut prev_num = data[0];
                let mut had_mistake = false;

                for &num in &data[1..] {
                    let bad_order = num.cmp(&prev_num) != order;
                    let bad_diff = num.abs_diff(prev_num) > 3;

                    if bad_order || bad_diff {
                        if had_mistake {
                            return false;
                        }
                        had_mistake = true;
                    } else {
                        prev_num = num;
                    }
                }
                true
            };

            if check_row(&row, Ordering::Less) || check_row(&row, Ordering::Greater) {
                return true;
            }
            row.reverse();

            check_row(&row, Ordering::Less) || check_row(&row, Ordering::Greater)
        })
        .count() as u32
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;

    const EXAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE.as_bytes()), 2);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE.as_bytes()), 4);
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
