#![feature(test)]
extern crate test;

use std::cmp::Ordering;

use bstr::ByteSlice;

const INPUT: &[u8] = include_bytes!("../../inputs/day01.txt");

fn parse_input(input: &[u8]) -> (Vec<u32>, Vec<u32>) {
    let (mut list1, mut list2): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| {
            let (num1, num2) = line.split_once_str("   ").unwrap();
            (
                unsafe { num1.to_str_unchecked() }.parse::<u32>().unwrap(),
                unsafe { num2.to_str_unchecked() }.parse::<u32>().unwrap(),
            )
        })
        .unzip();

    list1.sort_unstable();
    list2.sort_unstable();

    (list1, list2)
}

fn part_1(list1: &[u32], list2: &[u32]) -> u32 {
    list1
        .iter()
        .zip(list2)
        .map(|(&a, &b)| a.abs_diff(b))
        .sum::<u32>()
}

fn part_2(list1: &[u32], list2: &[u32]) -> u32 {
    let mut iter1 = list1.iter().peekable();
    let mut iter2 = list2.iter().peekable();

    let mut sum = 0;
    while let (Some(&n1), Some(&n2)) = (iter1.peek(), iter2.peek()) {
        match n1.cmp(n2) {
            Ordering::Equal => {
                let mut n2_count = 0;
                while Some(&n2) == iter2.peek() {
                    n2_count += 1;
                    iter2.next();
                }

                while Some(&n1) == iter1.peek() {
                    sum += n1 * n2_count;
                    iter1.next();
                }
            }
            Ordering::Less => {
                iter1.next();
            }
            Ordering::Greater => {
                iter2.next();
            }
        }
    }
    sum
}

fn main() {
    let (list1, list2) = parse_input(INPUT);
    println!("Part 1: {}", part_1(&list1, &list2));
    println!("Part 2: {}", part_2(&list1, &list2));
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;

    const EXAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part_1() {
        let (list1, list2) = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_1(&list1, &list2), 11);
    }

    #[test]
    fn test_part_2() {
        let (list1, list2) = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_2(&list1, &list2), 31);
    }

    #[bench]
    fn bench_parse_input(b: &mut test::Bencher) {
        b.iter(|| parse_input(black_box(INPUT)));
    }

    #[bench]
    fn bench_part_1(b: &mut test::Bencher) {
        let (list1, list2) = parse_input(INPUT);
        b.iter(|| part_1(black_box(&list1), black_box(&list2)));
    }

    #[bench]
    fn bench_part_2(b: &mut test::Bencher) {
        let (list1, list2) = parse_input(INPUT);
        b.iter(|| part_2(black_box(&list1), black_box(&list2)));
    }
}
