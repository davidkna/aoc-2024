#![feature(test)]
extern crate test;

use bstr::ByteSlice;
use fnv::FnvHashSet;
use itertools::Itertools;
use rayon::prelude::*;

const INPUT: &[u8] = include_bytes!("../../inputs/day-06.txt");

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Scaffold,
}

fn parse_input(input: &[u8]) -> ((i32, i32, Direction), Vec<Vec<Tile>>) {
    let mut robot = (0, 0, Direction::North);

    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, &c)| match c {
                    b'#' => Tile::Scaffold,
                    b'.' => Tile::Empty,
                    b'^' => {
                        robot = (x as i32, y as i32, Direction::North);
                        Tile::Empty
                    }
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();

    (robot, map)
}

fn part_1_helper(mut robot: (i32, i32, Direction), map: &[Vec<Tile>]) -> FnvHashSet<(i32, i32)> {
    let mut visisted = FnvHashSet::default();
    visisted.insert((robot.0, robot.1));

    loop {
        let (x, y, dir) = robot;

        let (next_x, next_y) = match dir {
            Direction::North => (x, y - 1),
            Direction::East => (x + 1, y),
            Direction::South => (x, y + 1),
            Direction::West => (x - 1, y),
        };

        if next_x < 0 || next_y < 0 {
            return visisted;
        }

        match map
            .get(next_y as usize)
            .and_then(|row| row.get(next_x as usize))
        {
            Some(Tile::Empty) => {
                visisted.insert((next_x, next_y));
                robot = (next_x, next_y, dir);
            }
            Some(Tile::Scaffold) => {
                robot = (x, y, dir.turn_right());
            }
            None => return visisted,
        }
    }
}

fn part_1(robot: (i32, i32, Direction), map: &[Vec<Tile>]) -> u32 {
    let visisted = part_1_helper(robot, map);

    visisted.len() as u32
}

fn robot_is_loop(mut robot: (i32, i32, Direction), map: &[Vec<Tile>]) -> bool {
    let mut visisted = FnvHashSet::default();
    visisted.insert(robot);

    loop {
        let (x, y, dir) = robot;

        let (next_x, next_y) = match dir {
            Direction::North => (x, y - 1),
            Direction::East => (x + 1, y),
            Direction::South => (x, y + 1),
            Direction::West => (x - 1, y),
        };

        if next_x < 0 || next_y < 0 {
            return false;
        }

        match map
            .get(next_y as usize)
            .and_then(|row| row.get(next_x as usize))
        {
            Some(Tile::Empty) => {
                robot = (next_x, next_y, dir);
            }
            Some(Tile::Scaffold) => {
                robot = (x, y, dir.turn_right());
            }
            None => return false,
        }

        if !visisted.insert(robot) {
            return true;
        }
    }
}

fn part_2(robot: (i32, i32, Direction), map: &[Vec<Tile>]) -> u32 {
    let (robot_x, robot_y, _) = robot;

    let eligible_points = part_1_helper(robot, map)
        .into_iter()
        .filter(|&(x, y)| {
            (x, y) != (robot_x, robot_y)
                && map
                    .get(y as usize)
                    .map_or(false, |row| row.get(x as usize) == Some(&Tile::Empty))
        })
        .collect_vec();

    eligible_points
        .into_par_iter()
        .filter(|&(x, y)| {
            let mut map_copy = map.to_vec();
            map_copy[y as usize][x as usize] = Tile::Scaffold;

            robot_is_loop(robot, &map_copy)
        })
        .count() as u32
}

fn main() {
    let (robot, map) = parse_input(INPUT);
    println!("Part 1: {}", part_1(robot, &map));
    println!("Part 2: {}", part_2(robot, &map));
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;

    const EXAMPLE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part_1() {
        let (robot, map) = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_1(robot, &map), 41);
    }

    #[test]
    fn test_part_2() {
        let (robot, map) = parse_input(EXAMPLE.as_bytes());

        assert_eq!(part_2(robot, &map), 6);
    }

    #[bench]
    fn bench_part_1(b: &mut test::Bencher) {
        let (robot, map) = parse_input(INPUT);
        b.iter(|| part_1(black_box(robot), black_box(&map)));
    }
    #[bench]
    fn bench_part_2(b: &mut test::Bencher) {
        let (robot, map) = parse_input(INPUT);

        b.iter(|| part_2(black_box(robot), black_box(&map)));
    }
}
