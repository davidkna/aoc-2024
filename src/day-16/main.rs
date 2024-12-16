#![feature(test)]
extern crate test;

use std::{cmp::Reverse, collections::BinaryHeap};

use bstr::ByteSlice;
use fnv::FnvHashSet;

const INPUT: &[u8] = include_bytes!("../../inputs/day-16.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
enum Dir {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Dir {
    fn turn_clockwise(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    fn turn_counter_clockwise(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }

    fn delta(&self) -> (i32, i32) {
        match self {
            Self::North => (0, -1),
            Self::East => (1, 0),
            Self::South => (0, 1),
            Self::West => (-1, 0),
        }
    }
}

fn solve(input: &[u8]) -> (u32, u32) {
    let width = input.lines().next().unwrap().len() as u32;
    let height = input.len() as u32 / (width + 1);

    let (sx, sy) = (1, width - 2);
    let (ex, ey) = (width - 2, 1);

    let mut heap = BinaryHeap::new();
    let mut visited = vec![vec![[None; 4]; width as usize]; height as usize];

    // Move 1 Points
    // turn 1000 Points

    let estim_cost = |(x, y): (u32, u32)| -> Reverse<u32> {
        // a) Minimum number of turns to face the target
        // b) Minimum number of moves to reach the target

        let turns = {
            let dx = ex as i32 - x as i32;
            let dy = ey as i32 - y as i32;

            let mut turns = 0;
            if dx != 0 {
                turns += 1;
            }

            if dy != 0 {
                turns += 1;
            }

            turns
        };

        let moves = {
            let dx = (ex as i32 - x as i32).abs();
            let dy = (ey as i32 - y as i32).abs();

            dx as u32 + dy as u32
        };

        Reverse(moves + 1000 * turns)
    };

    heap.push((estim_cost((sx, sy)), 0, sx, sy, Dir::East, vec![(sx, sy)]));
    let mut final_cost = None;
    let mut path_tiles = FnvHashSet::default();
    while let Some((_, cost, x, y, dir, path)) = heap.pop() {
        if (x, y) == (ex, ey) {
            if final_cost.is_none() {
                final_cost = Some(cost);
            }

            if final_cost == Some(cost) {
                path_tiles.extend(path);
            }
            continue;
        }

        if let Some(x) = visited[y as usize][x as usize][dir as usize] {
            if x < cost {
                continue;
            }
        }
        visited[y as usize][x as usize][dir as usize] = Some(cost);
        let mut move_it = |step_dir: Dir, step_cost| {
            let (dx, dy) = step_dir.delta();
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            let nidx = (ny as u32 * (width + 1) + nx as u32) as usize;

            // Check for wall
            if input[nidx] == b'#' {
                return;
            }

            if visited[ny as usize][nx as usize][step_dir as usize]
                .is_none_or(|c| c >= cost + step_cost)
            {
                let estim_dist_next =
                    Reverse(estim_cost((nx as u32, ny as u32)).0 + cost + step_cost);
                let mut p = path.clone();
                p.push((nx as u32, ny as u32));

                heap.push((
                    estim_dist_next,
                    cost + step_cost,
                    nx as u32,
                    ny as u32,
                    step_dir,
                    p,
                ));
            }
        };

        // Move Forward
        move_it(dir, 1);
        move_it(dir.turn_clockwise(), 1 + 1000);
        move_it(dir.turn_counter_clockwise(), 1 + 1000);
    }
    (final_cost.unwrap(), path_tiles.len() as u32)
}

fn part_1(input: &[u8]) -> u32 {
    solve(input).0
}

fn part_2(input: &[u8]) -> u32 {
    solve(input).1
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;

    const EXAMPLE_01: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const EXAMPLE_02: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE_01.as_bytes()), 7036);
        assert_eq!(part_1(EXAMPLE_02.as_bytes()), 11048);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE_01.as_bytes()), 45);
        assert_eq!(part_2(EXAMPLE_02.as_bytes()), 64);
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
