#![feature(test)]
extern crate test;

use std::fmt::Write;

use bstr::ByteSlice;
use itertools::Itertools;

const INPUT: &[u8] = include_bytes!("../../inputs/day-19.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Red = 3,
    White = 4,
}

const COLOR_CNT: usize = 5;

impl From<u8> for Color {
    fn from(c: u8) -> Self {
        match c {
            b'w' => Color::White,
            b'u' => Color::Blue,
            b'b' => Color::Black,
            b'r' => Color::Red,
            b'g' => Color::Green,
            _ => unreachable!("Invalid color"),
        }
    }
}

fn part_1(input: &[u8]) -> u32 {
    let (patterns, designs) = input.split_str("\n\n").collect_tuple().unwrap();
    // Transform the patterns into a regex
    let mut re_pattern = patterns
        .split_str(", ")
        .fold(String::from("^(?:"), |mut acc, pattern| {
            let _ = write!(acc, "(?:{})|", unsafe { pattern.to_str_unchecked() });
            acc
        });
    re_pattern.pop();
    re_pattern.push_str(")+$");

    let re = regex::bytes::Regex::new(&re_pattern).unwrap();
    let mut count = 0;
    for design in designs.lines() {
        if re.is_match(design) {
            count += 1;
        }
    }
    count
}

fn check_design(
    cursor: &[Color],
    color_map: &[Vec<Vec<Color>>; COLOR_CNT],
    cache: &mut [Option<u64>],
) -> u64 {
    if let Some(c) = cache[cursor.len()] {
        return c;
    }

    let initial_color = cursor.first().unwrap();

    let mut count = 0;
    for pattern in &color_map[*initial_color as usize] {
        if pattern.len() > cursor.len() {
            continue;
        }

        if let Some(rest) = cursor[1..].strip_prefix(pattern.as_slice()) {
            if rest.is_empty() {
                count += 1;
            } else if let Some(cached) = cache[rest.len()] {
                count += cached;
            } else {
                count += check_design(rest, color_map, cache);
            }
        }
    }
    cache[cursor.len()] = Some(count);
    count
}

fn part_2(input: &[u8]) -> u64 {
    let (patterns, designs) = input.split_str("\n\n").collect_tuple().unwrap();
    // Separate the patterns into a map of colors
    let color_map: [Vec<Vec<Color>>; COLOR_CNT] = std::array::from_fn(|_| vec![]);
    let color_map = patterns
        .split_str(", ")
        .fold(color_map, |mut acc, pattern| {
            let initial_color = Color::from(pattern[0]);
            let rest = pattern[1..]
                .iter()
                .map(|&c| Color::from(c))
                .collect::<Vec<_>>();
            acc[initial_color as usize].push(rest);

            acc
        });

    designs
        .lines()
        .map(|line| line.iter().copied().map(Color::from).collect_vec())
        .map(|design| {
            let mut cache = vec![None; design.len() + 1];
            check_design(design.as_slice(), &color_map, &mut cache)
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

    const EXAMPLE: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE.as_bytes()), 6);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE.as_bytes()), 16);
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
