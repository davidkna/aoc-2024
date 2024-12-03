#![feature(test)]
extern crate test;

use bstr::ByteSlice;

const INPUT: &[u8] = include_bytes!("../../inputs/day-03.txt");

fn part_1(input: &[u8]) -> u32 {
    input
        .find_iter("mul(")
        .map(|start| {
            let mut first_digit = 0;
            let mut continue_at = None;
            for (i, &c) in input.iter().enumerate().skip(start + 4) {
                match c {
                    b'0'..=b'9' => {
                        first_digit = first_digit * 10 + (c - b'0') as u32;
                    }
                    b',' => {
                        continue_at = Some(i + 1);
                        break;
                    }
                    _ => break,
                }
            }

            let Some(continue_at) = continue_at else {
                return 0;
            };

            let mut second_digit = 0;
            for c in input[continue_at..].iter() {
                match c {
                    b'0'..=b'9' => {
                        second_digit = second_digit * 10 + (c - b'0') as u32;
                    }
                    b')' => {
                        return first_digit * second_digit;
                    }
                    _ => break,
                }
            }
            0
        })
        .sum()
}
fn part_2(input: &[u8]) -> u32 {
    let dont_finder = bstr::Finder::new("don't()");
    let do_finder = bstr::Finder::new("do()");

    let mut idx = 0;
    let mut result = 0;
    loop {
        let Some(end) = dont_finder.find(&input[idx..]) else {
            result += part_1(&input[idx..]);
            break;
        };

        result += part_1(&input[idx..idx + end]);


        let end_of_chunk = idx + end + dont_finder.needle().len();
        let Some(do_start) = do_finder.find(&input[end_of_chunk..]) else {
            break;
        };
        idx = end_of_chunk + do_start + do_finder.needle().len();
    }
    result
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;

    const EXAMPLE_01: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const EXAMPLE_02: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE_01.as_bytes()), 161);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE_02.as_bytes()), 48);
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
