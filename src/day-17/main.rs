#![feature(test)]
extern crate test;

use std::collections::BinaryHeap;

use bstr::ByteSlice;

const INPUT: &[u8] = include_bytes!("../../inputs/day-17.txt");

enum Op {
    // Dvision
    Adv = 0,
    // Bitwise xor B register and literal
    Bxl = 1,
    // Combol modulo 8
    Bst = 2,
    // If A register is not zero, jump to literal
    Jnz = 3,
    // Bitwise and B register and register C
    // ignore literal
    Bxc = 4,
    // Output combo op module 8
    Out = 5,
    // Division, result in B register
    Bdv = 6,
    // Dvision, result in C register
    Cdv = 7,
}

fn parse_digits(s: &[u8]) -> u64 {
    s.iter().fold(0, |acc, &c| acc * 10 + u64::from(c - b'0'))
}

fn solve(instructions: &[u8], mut reg_a: u64, mut reg_b: u64, mut reg_c: u64) -> Vec<u8> {
    let mut output = Vec::new();
    let mut ip = 0;

    while let Some(&[ins, co]) = instructions.get(ip..=ip + 1) {
        let parsed_op = match ins {
            0 => Op::Adv,
            1 => Op::Bxl,
            2 => Op::Bst,
            3 => Op::Jnz,
            4 => Op::Bxc,
            5 => Op::Out,
            6 => Op::Bdv,
            7 => Op::Cdv,
            _ => unreachable!(),
        };

        let combo_value = match co {
            0..=3 => Some(u64::from(co)),
            4 => Some(reg_a),
            5 => Some(reg_b),
            6 => Some(reg_c),
            _ => None,
        };

        match parsed_op {
            Op::Adv => {
                reg_a >>= combo_value.unwrap();
            }
            Op::Bxl => {
                reg_b ^= u64::from(co);
            }
            Op::Bst => {
                reg_b = combo_value.unwrap() % 8;
            }
            Op::Jnz => {
                if reg_a != 0 {
                    ip = co as usize;
                    continue;
                }
            }
            Op::Bxc => {
                reg_b ^= reg_c;
            }
            Op::Out => {
                output.push((combo_value.unwrap() % 8) as u8);
            }
            Op::Bdv => {
                reg_b = reg_a >> combo_value.unwrap();
            }
            Op::Cdv => {
                reg_c = reg_a >> combo_value.unwrap();
            }
        }
        ip += 2;
    }

    output
}

fn parse_input(input: &[u8]) -> (Vec<u8>, u64, u64, u64) {
    let mut lines = input.lines();

    let reg_a = parse_digits(&lines.next().unwrap()[12..]);
    let reg_b = parse_digits(&lines.next().unwrap()[12..]);
    let reg_c = parse_digits(&lines.next().unwrap()[12..]);
    let instructions = lines.nth(1).unwrap()[9..]
        .split_str(",")
        .map(|x| (x[0] - b'0'))
        .collect::<Vec<_>>();

    (instructions, reg_a, reg_b, reg_c)
}

fn part_1(input: &[u8]) -> String {
    let (instructions, reg_a, reg_b, reg_c) = parse_input(input);

    solve(&instructions, reg_a, reg_b, reg_c)
        .iter()
        .map(u8::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct PotentialState {
    reg_a: u64,
    step: u64,
}

impl Ord for PotentialState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .step
            .cmp(&self.step)
            .then_with(|| self.reg_a.cmp(&other.reg_a))
    }
}

impl PartialOrd for PotentialState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn part_2(input: &[u8]) -> u64 {
    let (instructions, _, reg_b, reg_c) = parse_input(input);
    let mut queue = BinaryHeap::new();
    queue.push(PotentialState { reg_a: 0, step: 0 });

    while let Some(PotentialState { reg_a, step }) = queue.pop() {
        for j in 0..8 {
            let cand_a = (reg_a << 3) | j;
            let cand_solution = solve(&instructions, cand_a, reg_b, reg_c);

            // Find number of common elements (suffix)
            let common = std::iter::zip(cand_solution.iter().rev(), instructions.iter().rev())
                .take_while(|&(a, b)| a == b)
                .count();

            if common <= step as usize {
                continue;
            }
            if common == instructions.len() {
                return cand_a;
            }

            queue.push(PotentialState {
                reg_a: cand_a,
                step: common as u64,
            });
        }
    }

    unreachable!()
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;

    const EXAMPLE_1: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const EXAMPLE_2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE_1.as_bytes()), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE_2.as_bytes()), 117440);
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
