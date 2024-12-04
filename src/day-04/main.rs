#![feature(test)]
extern crate test;

use bstr::ByteSlice;
use itertools::Itertools;

const INPUT: &[u8] = include_bytes!("../../inputs/day-04.txt");

#[inline]
fn count_xmas(input: &[&[u8]], x: isize, y: isize) -> u32 {
    if input[y as usize][x as usize] != b'X' {
        return 0;
    }

    [-1, 0, 1]
        .into_iter()
        .cartesian_product([-1, 0, 1])
        .map(|(delta_x, delta_y)| {
            if delta_x == 0 && delta_y == 0 {
                return 0;
            }

            u32::from(
                std::iter::zip(
                    (1..).map(|i| x + delta_x * i),
                    (1..).map(|i| y + delta_y * i),
                )
                .take(3)
                .filter_map(|(xi, yi)| input.get(yi as usize)?.get(xi as usize))
                .zip_longest(b"MAS".iter())
                .all(|pair| match pair {
                    itertools::EitherOrBoth::Both(&c, &m) => c == m,
                    _ => false,
                }),
            )
        })
        .sum::<u32>()
}

fn part_1(input: &[u8]) -> u32 {
    let map = input.lines().collect::<Vec<_>>();
    let rows = map.len() as isize;
    let cols = map[0].len() as isize;

    (0..rows)
        .flat_map(|y| (0..cols).map(move |x| (x, y)))
        .map(|(x, y)| count_xmas(&map, x, y))
        .sum()
}

#[inline]
fn count_mas(input: &[&[u8]], x: isize, y: isize) -> bool {
    if input[y as usize][x as usize] != b'A' {
        return false;
    }

    let ul = input[y as usize - 1][x as usize - 1];
    let ur = input[y as usize - 1][x as usize + 1];
    let ll = input[y as usize + 1][x as usize - 1];
    let lr = input[y as usize + 1][x as usize + 1];

    let x1 = (ul == b'M' && lr == b'S') || (ul == b'S' && lr == b'M');
    let x2 = (ur == b'M' && ll == b'S') || (ur == b'S' && ll == b'M');

    x1 && x2
}

fn part_2(input: &[u8]) -> u32 {
    let map = input.lines().collect::<Vec<_>>();
    let rows = map.len() as isize;
    let cols = map[0].len() as isize;

    (1..rows - 1)
        .flat_map(|y| (1..cols - 1).map(move |x| (x, y)))
        .filter(|&(x, y)| count_mas(&map, x, y))
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

    const EXAMPLE_01: &str = "....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX
";
    const EXAMPLE_02: &str = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE_01.as_bytes()), 18);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE_02.as_bytes()), 9);
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
