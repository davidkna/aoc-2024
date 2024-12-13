#![feature(test)]
extern crate test;

use bstr::ByteSlice;

const INPUT: &[u8] = include_bytes!("../../inputs/day-13.txt");

fn parse_digits(s: &[u8]) -> i64 {
    s.iter().fold(0, |acc, &c| acc * 10 + (c - b'0') as i64)
}

fn solve(input: &[u8], part2: bool) -> u64 {
    input
        .split_str("\n\n")
        .map(|section| {
            let mut lines = section.lines();

            let button_a = unsafe { lines.next().unwrap_unchecked() };
            let (button_a_x, button_a_y) = {
                let x = unsafe {
                    10 * (button_a.get_unchecked(12) - b'0') + (button_a.get_unchecked(13) - b'0')
                } as i64;
                let y = unsafe {
                    10 * (button_a.get_unchecked(18) - b'0') + (button_a.get_unchecked(19) - b'0')
                } as i64;

                (x, y)
            };

            let button_b = unsafe { lines.next().unwrap_unchecked() };
            let (button_b_x, button_b_y) = {
                let x = unsafe {
                    10 * (button_b.get_unchecked(12) - b'0') + (button_b.get_unchecked(13) - b'0')
                } as i64;
                let y = unsafe {
                    10 * (button_b.get_unchecked(18) - b'0') + (button_b.get_unchecked(19) - b'0')
                } as i64;

                (x, y)
            };

            let (target_x, target_y) = {
                let target = unsafe { lines.next().unwrap_unchecked() };
                let (l, r) = unsafe { target.split_once_str(", ").unwrap_unchecked() };

                let tx = parse_digits(unsafe { &l.get_unchecked(9..) });
                let ty = parse_digits(r);

                if part2 {
                    (tx + 10000000000000, ty + 10000000000000)
                } else {
                    (tx, ty)
                }
            };

            // x_1 * button_a_x + x_2 * button_b_x = target_x
            // x_1 * button_a_y + x_2 * button_b_y = target_y

            // Cramer's rule
            let det = button_a_x * button_b_y - button_a_y * button_b_x;
            if det == 0 {
                return 0;
            }

            let x_1 = (target_x * button_b_y - target_y * button_b_x) / det;
            let x_2 = (button_a_x * target_y - button_a_y * target_x) / det;

            if !part2 && (x_1 > 100 || x_2 > 100) {
                return 0;
            }

            // Verify the solution to account for non-integer solutions
            if x_1 * button_a_x + x_2 * button_b_x != target_x
                || x_1 * button_a_y + x_2 * button_b_y != target_y
            {
                return 0;
            }

            x_1 as u64 * 3 + x_2 as u64
        })
        .sum()
}

fn part_1(input: &[u8]) -> u64 {
    solve(input, false)
}

fn part_2(input: &[u8]) -> u64 {
    solve(input, true)
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;

    const EXAMPLE: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE.as_bytes()), 480);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE.as_bytes()), 875318608908);
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
