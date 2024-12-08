#![feature(test)]
extern crate test;

use bstr::ByteSlice;
use fnv::FnvHashMap;
use itertools::Itertools;

const INPUT: &[u8] = include_bytes!("../../inputs/day-08.txt");

type Antennas = FnvHashMap<u8, Vec<(usize, usize)>>;

fn parse_input(input: &[u8]) -> (Vec<&[u8]>, Antennas) {
    let map = input.lines().collect_vec();

    let antennas = (0..map.len()).cartesian_product(0..map[0].len()).fold(
        FnvHashMap::default(),
        |mut acc: Antennas, (y, x)| {
            match map[y][x] {
                b'.' => (),
                antenna => {
                    acc.entry(antenna).or_default().push((y, x));
                }
            };
            acc
        },
    );

    (map, antennas)
}

fn part_1(input: &[u8]) -> u64 {
    let (map, antennas) = parse_input(input);

    antennas
        .iter()
        .flat_map(|(_freq, antennas)| {
            antennas.iter().tuple_combinations().flat_map(|(a, b)| {
                let (a_y, a_x) = *a;
                let (b_y, b_x) = *b;

                let dx = a_x as i32 - b_x as i32;
                let dy = a_y as i32 - b_y as i32;

                let axp = usize::try_from(a_x as i32 + dx);
                let ayp = usize::try_from(a_y as i32 + dy);

                let bxp = usize::try_from(b_x as i32 - dx);
                let byp = usize::try_from(b_y as i32 - dy);

                [(ayp.ok(), axp.ok()), (byp.ok(), bxp.ok())]
                    .into_iter()
                    .filter_map(|(yo, xo)| {
                        let (y, x) = (yo?, xo?);
                        map.get(y)?.get(x).map(|_| (y, x))
                    })
            })
        })
        .unique()
        .count() as u64
}

fn test_locations(
    map: &[&[u8]],
    (mut y, mut x): (usize, usize),
    (dy, dx): (i32, i32),
) -> Vec<(usize, usize)> {
    let mut locations = vec![(y, x)];

    loop {
        let mut t = || {
            y = usize::try_from(y as i32 + dy).ok()?;
            x = usize::try_from(x as i32 + dx).ok()?;

            map.get(y)?.get(x).map(|_| (y, x))
        };

        let Some((y, x)) = t() else {
            break;
        };

        locations.push((y, x));
    }

    locations
}

fn part_2(input: &[u8]) -> u64 {
    let (map, antennas) = parse_input(input);

    antennas
        .iter()
        .flat_map(|(_freq, antennas)| {
            antennas.iter().tuple_combinations().flat_map(|(a, b)| {
                let (a_y, a_x) = *a;
                let (b_y, b_x) = *b;

                let dx = a_x as i32 - b_x as i32;
                let dy = a_y as i32 - b_y as i32;

                test_locations(&map, (a_y, a_x), (dy, dx))
                    .into_iter()
                    .chain(test_locations(&map, (b_y, b_x), (-dy, -dx)))
            })
        })
        .unique()
        .count() as u64
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;

    const EXAMPLE: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE.as_bytes()), 14);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE.as_bytes()), 34);
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
