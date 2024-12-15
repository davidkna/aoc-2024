#![feature(test)]
extern crate test;

use bstr::ByteSlice;

const INPUT: &[u8] = include_bytes!("../../inputs/day-15.txt");

fn part_1(input: &[u8]) -> u32 {
    let (map, instructions) = input.split_once_str("\n\n").unwrap();
    let mut map = map.lines().map(|line| line.to_vec()).collect::<Vec<_>>();

    let mut pos = (0, 0);
    for (y, line) in map.iter().enumerate() {
        if let Some(x) = line.iter().position(|&c| c == b'@') {
            pos = (y, x);
            map[y][x] = b'.';
            break;
        }
    }

    'next_ins: for ins in instructions {
        let delta = match ins {
            b'^' => (-1, 0),
            b'v' => (1, 0),
            b'<' => (0, -1),
            b'>' => (0, 1),
            b'\n' => continue,
            _ => unreachable!(),
        };
        let next_pos = (pos.0 as isize + delta.0, pos.1 as isize + delta.1);
        let mut cursor = next_pos;
        loop {
            match map[cursor.0 as usize][cursor.1 as usize] {
                b'#' => continue 'next_ins,
                b'.' => {
                    map[cursor.0 as usize][cursor.1 as usize] = b'O';
                    break;
                }
                b'O' => {
                    cursor.0 += delta.0;
                    cursor.1 += delta.1;
                }
                _ => unreachable!(),
            }
        }
        pos = (next_pos.0 as usize, next_pos.1 as usize);
        map[pos.0][pos.1] = b'.';
    }

    map.iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(x, &c)| (c == b'O').then_some((100 * y + x) as u32))
        })
        .sum::<u32>()
}

fn can_move(
    map: &Vec<Vec<u8>>,
    pos: (usize, usize),
    delta: (isize, isize),
    visited: &mut Vec<Vec<Option<bool>>>,
) -> bool {
    let next_pos = (pos.0 as isize + delta.0, pos.1 as isize + delta.1);
    if let Some(result) = visited[next_pos.0 as usize][next_pos.1 as usize] {
        return result;
    }

    let tile = map[next_pos.0 as usize][next_pos.1 as usize];
    let is_vertical = delta.0 != 0;
    let result = match tile {
        b'#' => false,
        b'[' if is_vertical => {
            can_move(
                map,
                (next_pos.0 as usize, next_pos.1 as usize),
                delta,
                visited,
            ) && can_move(
                map,
                (next_pos.0 as usize, (next_pos.1 + 1) as usize),
                delta,
                visited,
            )
        }
        b']' if is_vertical => {
            can_move(
                map,
                (next_pos.0 as usize, next_pos.1 as usize),
                delta,
                visited,
            ) && can_move(
                map,
                (next_pos.0 as usize, (next_pos.1 - 1) as usize),
                delta,
                visited,
            )
        }
        b'[' | b']' => can_move(
            map,
            (next_pos.0 as usize, next_pos.1 as usize),
            delta,
            visited,
        ),
        b'.' => true,
        _ => unreachable!(),
    };

    visited[next_pos.0 as usize][next_pos.1 as usize] = Some(result);
    result
}

fn do_move(map: &mut Vec<Vec<u8>>, pos: (usize, usize), delta: (isize, isize), what: u8) {
    let next_pos = (pos.0 as isize + delta.0, pos.1 as isize + delta.1);
    let tile = map[next_pos.0 as usize][next_pos.1 as usize];
    map[next_pos.0 as usize][next_pos.1 as usize] = what;
    let is_vertical = delta.0 != 0;

    match tile {
        b'[' if is_vertical => {
            map[next_pos.0 as usize][(next_pos.1 + 1) as usize] = b'.';
            do_move(map, (next_pos.0 as usize, next_pos.1 as usize), delta, b'[');
            do_move(
                map,
                (next_pos.0 as usize, (next_pos.1 + 1) as usize),
                delta,
                b']',
            );
        }
        b']' if is_vertical => {
            map[next_pos.0 as usize][(next_pos.1 - 1) as usize] = b'.';
            do_move(map, (next_pos.0 as usize, next_pos.1 as usize), delta, b']');
            do_move(
                map,
                (next_pos.0 as usize, (next_pos.1 - 1) as usize),
                delta,
                b'[',
            );
        }
        b'[' | b']' => {
            do_move(map, (next_pos.0 as usize, next_pos.1 as usize), delta, tile);
        }
        b'.' => (),
        _ => unsafe { std::hint::unreachable_unchecked() },
    };
}

fn part_2(input: &[u8]) -> u32 {
    let (map, instructions) = input.split_once_str("\n\n").unwrap();
    let mut map = map
        .lines()
        .map(|line| {
            line.iter()
                .flat_map(|&c| match c {
                    b'#' => [b'#', b'#'],
                    b'@' => [b'@', b'.'],
                    b'O' => [b'[', b']'],
                    b'.' => [b'.', b'.'],
                    _ => unsafe { std::hint::unreachable_unchecked() },
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut pos = (0, 0);
    for (y, line) in map.iter().enumerate() {
        if let Some(x) = line.iter().position(|&c| c == b'@') {
            pos = (y, x);
            map[y][x] = b'.';
            break;
        }
    }

    for ins in instructions {
        let delta = match ins {
            b'^' => (-1, 0),
            b'v' => (1, 0),
            b'<' => (0, -1),
            b'>' => (0, 1),
            b'\n' => continue,
            _ => unreachable!(),
        };
        let mut visited = vec![vec![None; map[0].len()]; map.len()];
        if can_move(&map, pos, delta, &mut visited) {
            do_move(&mut map, pos, delta, b'.');
            pos = (
                (pos.0 as isize + delta.0) as usize,
                (pos.1 as isize + delta.1) as usize,
            );
        }
    }

    map.iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(x, &c)| (c == b'[').then_some((100 * y + x) as u32))
        })
        .sum::<u32>()
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;

    const EXAMPLE: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE.as_bytes()), 10092);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE.as_bytes()), 9021);
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
