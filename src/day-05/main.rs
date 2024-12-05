#![feature(test)]
extern crate test;

use bstr::ByteSlice;
use itertools::Itertools;

const INPUT: &[u8] = include_bytes!("../../inputs/day-05.txt");

fn part_1(input: &[u8]) -> u32 {
    let (rules, instructions) = input.split_once_str("\n\n").unwrap();

    let rules = rules.lines().fold([[false; 100]; 100], |mut acc, line| {
        let (before, after) = line.split_once_str("|").unwrap();
        let (before, after) = (
            unsafe { before.to_str_unchecked() }
                .parse::<usize>()
                .unwrap(),
            unsafe { after.to_str_unchecked() }
                .parse::<usize>()
                .unwrap(),
        );
        acc[before][after] = true;
        acc
    });

    instructions
        .lines()
        .filter_map(|line| {
            let mut acc: Vec<usize> = vec![];

            let ins = line
                .split_str(",")
                .map(|n| unsafe { n.to_str_unchecked() }.parse::<usize>().unwrap())
                .collect_vec();

            ins.iter()
                .all(|&n| {
                    let cond = acc.iter().all(|&m| rules[m][n]);
                    acc.push(n);
                    cond
                })
                .then_some(ins[ins.len() / 2] as u32)
        })
        .sum()
}

fn toposort(items: &[usize], rules: &[[bool; 100]; 100]) -> Vec<usize> {
    let mut visited = [false; 100];
    let mut stack = Vec::with_capacity(items.len());
    let mut result = vec![];

    fn dfs(
        node: usize,
        visited: &mut [bool; 100],
        stack: &mut Vec<usize>,
        rules: &[[bool; 100]; 100],
    ) {
        visited[node] = true;

        for (i, &rule) in (0..100).zip(rules[node].iter()) {
            if rule && !visited[i] {
                dfs(i, visited, stack, rules);
            }
        }

        stack.push(node);
    }

    for &item in items {
        if !visited[item] {
            dfs(item, &mut visited, &mut stack, rules);
        }
    }

    while let Some(item) = stack.pop() {
        result.push(item);
    }

    result
}

fn part_2(input: &[u8]) -> u32 {
    let (rules, instructions) = input.split_once_str("\n\n").unwrap();

    let rules = rules.lines().fold([[false; 100]; 100], |mut acc, line| {
        let (before, after) = line.split_once_str("|").unwrap();
        let (before, after) = (
            unsafe { before.to_str_unchecked() }
                .parse::<usize>()
                .unwrap(),
            unsafe { after.to_str_unchecked() }
                .parse::<usize>()
                .unwrap(),
        );
        acc[before][after] = true;
        acc
    });

    let sorted = {
        let items: [usize; 100] = std::array::from_fn(|i| i);
        toposort(&items, &rules)
    };

    let sorted_pos = sorted
        .into_iter()
        .enumerate()
        .fold([0; 100], |mut acc, (i, n)| {
            acc[n] = i;
            acc
        });

    instructions
        .lines()
        .filter_map(|line| {
            let mut acc: Vec<usize> = vec![];

            let mut ins = line
                .split_str(",")
                .map(|n| unsafe { n.to_str_unchecked() }.parse::<usize>().unwrap())
                .collect_vec();

            ins.iter()
                .any(|&n| {
                    let cond = acc.iter().all(|&m| rules[m][n]);
                    acc.push(n);
                    !cond
                })
                .then(|| {
                    let target_idx = ins.len() / 2;
                    let (_, result, _) =
                        ins.select_nth_unstable_by_key(target_idx / 2, |&n| sorted_pos[n]);
                    *result as u32
                })
        })
        .sum()
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;

    const EXAMPLE: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE.as_bytes()), 143);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE.as_bytes()), 123);
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
