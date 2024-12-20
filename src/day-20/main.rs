#![feature(test)]
extern crate test;

use std::collections::VecDeque;

use bstr::ByteSlice;
use itertools::Itertools;

const INPUT: &[u8] = include_bytes!("../../inputs/day-20.txt");

fn fill_distances(grid: &[&[u8]], start: (usize, usize), distances: &mut Vec<Vec<i32>>) {
    let height = grid.len();
    let width = grid[0].len();
    let mut queue = VecDeque::new();
    queue.push_back((0, start));

    while let Some((distance, (y, x))) = queue.pop_front() {
        if distances[y][x] <= distance {
            continue;
        }
        distances[y][x] = distance;

        for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (ny, nx) = (y as i32 + dy, x as i32 + dx);
            if ny < 0 || ny >= height as i32 || nx < 0 || nx >= width as i32 {
                continue;
            }
            let (ny, nx) = (ny as usize, nx as usize);

            if grid[ny][nx] == b'#' {
                continue;
            }

            if distances[ny][nx] <= distance + 1 {
                continue;
            }

            queue.push_back((distance + 1, (ny, nx)));
        }
    }
}

fn solve<const MIN_SAVINGS: i32, const CHEAT_STEPS: i32>(input: &[u8]) -> u32 {
    let grid = input.lines().collect_vec();
    let height = grid.len();
    let width = grid[0].len();

    let start = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.find_byte(b'S').map(|x| (y, x)))
        .unwrap();

    let target = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.find_byte(b'E').map(|x| (y, x)))
        .unwrap();

    // Walk grid and store min distance to target for each cell
    let mut distances_to_end = vec![vec![i32::MAX; width]; height];
    let mut distances_to_start = vec![vec![i32::MAX; width]; height];
    fill_distances(&grid, target, &mut distances_to_end);
    fill_distances(&grid, start, &mut distances_to_start);

    let base_score = distances_to_end[start.0][start.1];

    let mut count = 0;
    for y in 1..grid.len() - 1 {
        for x in 1..grid[0].len() - 1 {
            let current_cell = grid[y][x];
            if current_cell == b'#' || distances_to_end[y][x] == i32::MAX {
                continue;
            }

            let base_steps = distances_to_start[y][x];
            for dy in -CHEAT_STEPS..=CHEAT_STEPS {
                let ny = y as i32 + dy;
                if ny < 0 || ny >= height as i32 {
                    continue;
                }
                let ny = ny as usize;

                let remaining = CHEAT_STEPS - dy.abs();
                for dx in -remaining..=remaining {
                    let nx = x as i32 + dx;
                    if nx < 0 || nx >= width as i32 {
                        continue;
                    }
                    let nx = nx as usize;

                    if grid
                        .get(ny)
                        .is_none_or(|row| row.get(nx).is_none_or(|f| *f == b'#'))
                    {
                        continue;
                    }

                    if distances_to_end[ny][nx] == i32::MAX {
                        continue;
                    }

                    let new_distance = distances_to_end[ny][nx];

                    let cheated_distance = (y.abs_diff(ny) + x.abs_diff(nx)) as i32;

                    let score = base_steps + cheated_distance + new_distance;
                    let savings = base_score - score;

                    if savings >= MIN_SAVINGS {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn part_1<const MIN_SAVINGS: i32>(input: &[u8]) -> u32 {
    solve::<MIN_SAVINGS, 2>(input)
}

fn part_2<const MIN_SAVINGS: i32>(input: &[u8]) -> u32 {
    solve::<MIN_SAVINGS, 20>(input)
}

fn main() {
    println!("Part 1: {}", part_1::<100>(INPUT));
    println!("Part 2: {}", part_2::<100>(INPUT));
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;

    const EXAMPLE: &str = "r###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1::<20>(EXAMPLE.as_bytes()), 5);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2::<72>(EXAMPLE.as_bytes()), 29);
    }

    #[bench]
    fn bench_part_1(b: &mut test::Bencher) {
        b.iter(|| part_1::<100>(black_box(INPUT)));
    }

    #[bench]
    fn bench_part_2(b: &mut test::Bencher) {
        b.iter(|| part_2::<100>(black_box(INPUT)));
    }
}
