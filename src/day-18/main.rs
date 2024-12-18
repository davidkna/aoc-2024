#![feature(test)]
extern crate test;

use std::{cmp::Reverse, collections::BinaryHeap};

use bstr::ByteSlice;

const INPUT: &[u8] = include_bytes!("../../inputs/day-18.txt");

fn parse_digits(s: &[u8]) -> u32 {
    s.iter().fold(0, |acc, &c| acc * 10 + u32::from(c - b'0'))
}

fn solve<const GRIDSIZE: usize>(corruption: &[Vec<bool>]) -> Option<u32> {
    let start = (0, 0);
    let end = (GRIDSIZE - 1, GRIDSIZE - 1);

    let mut q = BinaryHeap::new();
    q.push((Reverse(2 * (GRIDSIZE - 1)), 0usize, (start)));

    let mut visited = vec![vec![false; GRIDSIZE]; GRIDSIZE];

    while let Some((_, dist, (x, y))) = q.pop() {
        if (x, y) == end {
            return Some(dist as u32);
        }

        if visited[y][x] {
            continue;
        }
        visited[y][x] = true;

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (Some(nx), Some(ny)) = (
                usize::try_from(x as i32 + dx).ok(),
                usize::try_from(y as i32 + dy).ok(),
            ) else {
                continue;
            };

            let Some(&grid_tile) = corruption.get(nx).and_then(|col| col.get(ny)) else {
                continue;
            };
            if grid_tile || visited[ny][nx] {
                continue;
            }
            q.push((
                Reverse(dist + 1 + (GRIDSIZE - 1 - nx) + (GRIDSIZE - 1 - ny)),
                dist + 1,
                (nx, ny),
            ));
        }
    }

    None
}

fn part_1<const GRIDSIZE: usize, const LIMIT: usize>(input: &[u8]) -> u32 {
    let corruption = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once_str(",").unwrap();
            (parse_digits(x), parse_digits(y))
        })
        .take(LIMIT)
        .fold(vec![vec![false; GRIDSIZE]; GRIDSIZE], |mut acc, (x, y)| {
            acc[y as usize][x as usize] = true;
            acc
        });

    solve::<GRIDSIZE>(&corruption).unwrap()
}

fn part_2<const GRIDSIZE: usize, const LIMIT: usize>(input: &[u8]) -> String {
    let corruption = input.lines().map(|line| {
        let (x, y) = line.split_once_str(",").unwrap();
        (parse_digits(x), parse_digits(y))
    });

    let mut curroption_map = vec![vec![false; GRIDSIZE]; GRIDSIZE];

    for (i, (x, y)) in corruption.enumerate() {
        curroption_map[y as usize][x as usize] = true;
        if i >= LIMIT && solve::<GRIDSIZE>(&curroption_map).is_none() {
            return format!("{x},{y}");
        }
    }
    unreachable!()
}

fn main() {
    println!("Part 1: {}", part_1::<71, 1024>(INPUT));
    println!("Part 2: {}", part_2::<71, 1024>(INPUT));
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;

    const EXAMPLE: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1::<7, 12>(EXAMPLE.as_bytes()), 22);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2::<7, 12>(EXAMPLE.as_bytes()), "6,1");
    }

    #[bench]
    fn bench_part_1(b: &mut test::Bencher) {
        b.iter(|| part_1::<70, 1024>(black_box(INPUT)));
    }

    #[bench]
    fn bench_part_2(b: &mut test::Bencher) {
        b.iter(|| part_2::<70, 1024>(black_box(INPUT)));
    }
}