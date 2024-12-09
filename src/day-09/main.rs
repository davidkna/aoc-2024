#![feature(test)]
extern crate test;

use std::collections::VecDeque;

use itertools::Itertools;

const INPUT: &[u8] = include_bytes!("../../inputs/day-09.txt");

fn parse_digit(c: u8) -> u64 {
    (c - b'0') as u64
}

#[derive(Clone, Copy)]
struct File {
    file_id: usize,
    size: u64,
    empty: u64,
}

fn part_1(input: &[u8]) -> u64 {
    let mut storage = input
        .chunks_exact(2)
        .enumerate()
        .map(|(file_id, chunk)| {
            let size = parse_digit(chunk[0]);
            let empty = parse_digit(chunk[1]);
            File {
                file_id,
                size,
                empty,
            }
        })
        .collect::<VecDeque<_>>();

    if input.len() % 2 != 0 {
        let size = parse_digit(*input.last().unwrap());
        let file_id = storage.len();

        storage.push_back(File {
            file_id,
            size,
            empty: 0,
        });
    }

    let mut result = 0;
    let mut idx = 0;
    while let Some(File {
        file_id,
        size,
        mut empty,
    }) = storage.pop_front()
    {
        for _ in 0..size {
            result += idx * file_id as u64;
            idx += 1;
        }

        while empty > 0 {
            let Some(last) = storage.back_mut() else {
                break;
            };
            if last.size == 0 {
                storage.pop_back().unwrap();
                continue;
            }

            last.size -= 1;
            result += idx * (last.file_id as u64);

            idx += 1;
            empty -= 1;
        }
    }
    result
}

fn part_2(input: &[u8]) -> u64 {
    let mut storage = input
        .chunks_exact(2)
        .enumerate()
        .map(|(file_id, chunk)| {
            let size = parse_digit(chunk[0]);
            let empty = parse_digit(chunk[1]);
            File {
                file_id,
                size,
                empty,
            }
        })
        .collect_vec();

    if input.len() % 2 != 0 {
        let size = parse_digit(*input.last().unwrap());
        let file_id = storage.len();

        storage.push(File {
            file_id,
            size,
            empty: 0,
        });
    }

    let mut file_order = vec![];
    for i in 0..storage.len() {
        file_order.push(i);
    }

    for file_id in (1..storage.len()).rev() {
        let file = storage[file_id];

        let Some(target) = storage
            .iter()
            .filter(|f| {
                f.file_id != file_id
                    && f.empty >= file.size
                    && file_order[f.file_id] < file_order[file_id]
            })
            .min_by_key(|f| file_order[f.file_id])
        else {
            continue;
        };
        let target_id: usize = target.file_id;

        let prev_id = file_order
            .iter()
            .enumerate()
            .find(|(_, &o)| o == file_order[file_id] - 1)
            .unwrap()
            .0;

        storage[prev_id].empty += file.size + file.empty;
        storage[file_id].empty = storage[target_id].empty - file.size;
        storage[target_id].empty = 0;

        // Update the file order
        for i in 0..file_order.len() {
            if file_order[i] > file_order[target_id] {
                file_order[i] += 1;
            }
        }

        file_order[file_id] = file_order[target_id] + 1;
    }

    storage.sort_unstable_by_key(|f| file_order[f.file_id]);

    let mut idx = 0;
    let mut result = 0;
    for File {
        file_id,
        size,
        empty,
    } in storage
    {
        for _ in 0..size {
            result += idx * file_id as u64;
            idx += 1;
        }

        idx += empty;
    }

    result
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;

    const EXAMPLE: &str = "2333133121414131402";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE.as_bytes()), 1928);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE.as_bytes()), 2858);
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
