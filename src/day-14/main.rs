#![feature(test)]
extern crate test;

use bstr::ByteSlice;
use cabac::traits::CabacWriter;
use itertools::Itertools;

const INPUT: &[u8] = include_bytes!("../../inputs/day-14.txt");

fn parse_int(s: &[u8]) -> i32 {
    match s {
        [b'-', rest @ ..] => -parse_uint(rest),
        _ => parse_uint(s),
    }
}

fn parse_uint(s: &[u8]) -> i32 {
    s.iter().fold(0, |acc, &c| acc * 10 + (c - b'0') as i32)
}

fn mod_inverse(a: i32, m: i32) -> i32 {
    let mut a = a.rem_euclid(m);
    let mut m = m;
    let mut x0 = 0;
    let mut x1 = 1;

    while a > 1 {
        let q = a / m;
        let t = m;

        m = a % m;
        a = t;

        let t = x0;
        x0 = x1 - q * x0;
        x1 = t;
    }

    if x1 < 0 {
        x1 + m
    } else {
        x1
    }
}

fn part_1(input: &[u8], lim_x: i32, lim_y: i32) -> u32 {
    input
        .lines()
        .fold([0u32; 4], |mut acc, line| {
            let (pos, vel) = line.split_once_str(" v=").unwrap();

            let (x0, y0) = {
                let (tx, ty) = pos[2..].split_once_str(",").unwrap();

                (parse_uint(tx), parse_uint(ty))
            };

            let (vx, vy) = {
                let (tx, ty) = vel.split_once_str(",").unwrap();

                (parse_int(tx), parse_int(ty))
            };
            let x1 = (x0 + 100 * vx).rem_euclid(lim_x);
            let y1 = (y0 + 100 * vy).rem_euclid(lim_y);
            if x1 == lim_x / 2 || y1 == lim_y / 2 {
                return acc;
            }

            let in_upper_half = y1 < lim_y / 2;
            let in_left_half = x1 < lim_x / 2;

            let quadrant = (in_upper_half as usize) << 1 | in_left_half as usize;

            acc[quadrant] += 1;

            acc
        })
        .iter()
        .product()
}

struct CountWriter(u32);

impl std::io::Write for CountWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0 += buf.len() as u32;

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn part_2(input: &[u8], lim_x: i32, lim_y: i32, print: bool) -> u32 {
    let mut robots = input
        .lines()
        .map(|line| {
            let (pos, vel) = line.split_once_str(" v=").unwrap();

            let (x0, y0) = {
                let (tx, ty) = pos[2..].split_once_str(",").unwrap();

                (parse_uint(tx), parse_uint(ty))
            };

            let (vx, vy) = {
                let (tx, ty) = vel.split_once_str(",").unwrap();

                (parse_int(tx), parse_int(ty))
            };

            ((x0, y0), (vx, vy))
        })
        .collect_vec();

    let steps = lim_x.max(lim_y);
    let min_entropy_x_idx = (1..=steps)
        .min_by_key(|_| {
            // Move Robots and mesure entropy
            let mut counter = CountWriter(0);
            let mut cabac_writer = cabac::h265::H265Writer::new(&mut counter);
            let mut grid = vec![false; (lim_x * lim_y) as usize];

            for ((x, y), (vx, _vy)) in &mut robots {
                *x = (*x + *vx).rem_euclid(lim_x);
                grid[(*y * lim_x + *x) as usize] = true;
            }

            let mut ctx = cabac::h265::H265Context::default();
            for &cell in grid.iter() {
                let _ = cabac_writer.put(cell, &mut ctx);
            }

            let _ = cabac_writer.finish();
            counter.0
        })
        .unwrap();
    // Y Axis
    let min_entropy_y_idx = (1..=steps)
        .min_by_key(|_| {
            // Move Robots and mesure entropy
            let mut counter = CountWriter(0);
            let mut cabac_writer = cabac::h265::H265Writer::new(&mut counter);
            let mut grid = vec![false; (lim_x * lim_y) as usize];

            for ((x, y), (_vx, vy)) in &mut robots {
                *y = (*y + *vy).rem_euclid(lim_y);
                grid[(*x * lim_y + *y) as usize] = true;
            }

            let mut ctx = cabac::h265::H265Context::default();
            for &cell in grid.iter() {
                let _ = cabac_writer.put(cell, &mut ctx);
            }

            let _ = cabac_writer.finish();
            counter.0
        })
        .unwrap();

    // Chinese Remainder Theorem
    let result = min_entropy_x_idx
        + ((min_entropy_y_idx - min_entropy_x_idx) * mod_inverse(lim_x, lim_y)).rem_euclid(lim_y)
            * lim_x;

    // Move Robots to the result
    if print {
        let steps_to_do = result - steps;

        let mut grid = vec![vec![false; lim_x as usize]; lim_y as usize];

        for ((x, y), (vx, vy)) in &mut robots {
            *x = (*x + *vx * steps_to_do).rem_euclid(lim_x);
            *y = (*y + *vy * steps_to_do).rem_euclid(lim_y);

            grid[*y as usize][*x as usize] = true;
        }

        // Print the grid
        for y in 0..lim_y {
            for x in 0..lim_x {
                print!(
                    "{}",
                    if grid[y as usize][x as usize] {
                        '#'
                    } else {
                        '.'
                    }
                );
            }
            println!();
        }
    }

    result as u32
}

fn main() {
    println!("Part 1: {}", part_1(INPUT, 101, 103));
    println!("Part 2: {}", part_2(INPUT, 101, 103, true));
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;

    const EXAMPLE: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE.as_bytes(), 11, 7), 12);
    }

    #[bench]
    fn bench_part_1(b: &mut test::Bencher) {
        b.iter(|| part_1(black_box(INPUT), 101, 103));
    }

    #[bench]
    fn bench_part_2(b: &mut test::Bencher) {
        b.iter(|| part_2(black_box(INPUT), 101, 103, false));
    }
}
