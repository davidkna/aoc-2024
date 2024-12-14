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

struct CountWriter(usize);

impl std::io::Write for CountWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0 += buf.len();

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn part_2(input: &[u8], lim_x: i32, lim_y: i32) -> u32 {
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

    for it in 1.. {
        // Move Robots and mesure entropy
        let mut ctx = cabac::h265::H265Context::default();
        let mut counter = CountWriter(0);
        let mut cabac_writer = cabac::h265::H265Writer::new(&mut counter);

        let mut grid = vec![vec![false; lim_x as usize]; lim_y as usize];

        for ((x, y), (vx, vy)) in &mut robots {
            *x = (*x + *vx).rem_euclid(lim_x);
            *y = (*y + *vy).rem_euclid(lim_y);

            grid[*y as usize][*x as usize] = true;
        }

        for &item in (grid).iter().flatten() {
            let _ = cabac_writer.put(item, &mut ctx);
        }

        let _ = cabac_writer.finish();

        let entropy = counter.0;

        if entropy < 300 {
            // Print the Grid
            for row in grid {
                for &cell in &row {
                    print!("{}", if cell { '#' } else { '.' });
                }
                println!();
            }

            return it;
        }
    }
    unreachable!()
}

fn main() {
    println!("Part 1: {}", part_1(INPUT, 101, 103));
    println!("Part 2: {}", part_2(INPUT, 101, 103));
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
        b.iter(|| part_2(black_box(INPUT), 101, 103));
    }
}
