#![feature(test)]
extern crate test;

use bstr::ByteSlice;
use itertools::Itertools;
use rayon::prelude::*;
use z3::ast::{Ast, Int};


const INPUT: &[u8] = include_bytes!("../../inputs/day-13.txt");

fn parse_digits(s: &[u8]) -> u64 {
    s.iter().fold(0, |acc, &c| acc * 10 + (c - b'0') as u64)
}

fn solve(input: &[u8], part2: bool) -> u64 {
    let cost_button_a = 3i64;
    let cost_button_b = 1i64;

    input.split_str("\n\n")
        .collect_vec()
        .into_par_iter()
        .map(|section| {
            let lines = section.lines();
            let (button_a, button_b, prize) = lines.collect_tuple().unwrap();

            let (target_x, target_y) = {
                let (l,r) =prize
                    .split_once_str(", ")
                    .unwrap();

                let (_, x_str) = l.split_once_str("=").unwrap();
                let (_, y_str) = r.split_once_str("=").unwrap();

                let (tx, ty) = (parse_digits(x_str), parse_digits(y_str));

                if part2 {
                    (tx + 10000000000000, ty + 10000000000000)
                } else {
                    (tx, ty)
                }
            };

            let (button_a_x, button_a_y) = {
                let (l,r) = button_a
                    .split_once_str(", ")
                    .unwrap();

                let (_, x_str) = l.split_once_str("+").unwrap();
                let (_, y_str) = r.split_once_str("+").unwrap();

                (parse_digits(x_str), parse_digits(y_str))
            };

            let (button_b_x, button_b_y) = {
                let (l,r) = button_b
                    .split_once_str(", ")
                    .unwrap();

                let (_, x_str) = l.split_once_str("+").unwrap();
                let (_, y_str) = r.split_once_str("+").unwrap();

                (parse_digits(x_str), parse_digits(y_str))
            };

            let ctx = z3::Context::new(&z3::Config::default());
            let solver = z3::Optimize::new(&ctx);

            let (
                buttons_pressed_a,
                buttons_pressed_b,
            ) = (
                Int::new_const(&ctx, "buttons_pressed_a"),
                Int::new_const(&ctx, "buttons_pressed_b"),
            );
            solver.assert(&buttons_pressed_a.ge(&Int::from_i64(&ctx, 0)));
            solver.assert(&buttons_pressed_b.ge(&Int::from_i64(&ctx, 0)));

            if !part2 {
                solver.assert(&((&buttons_pressed_a)).le(&Int::from_i64(&ctx, 100i64)));
                solver.assert(&((&buttons_pressed_b)).le(&Int::from_i64(&ctx, 100i64)));
            }
            solver.assert(
                &((&buttons_pressed_a * button_a_x + &buttons_pressed_b * button_b_x)._eq(&Int::from_i64(&ctx, target_x as _))),
            );
            solver.assert(
                &((&buttons_pressed_a * button_a_y + &buttons_pressed_b * button_b_y)._eq(&Int::from_i64(&ctx, target_y as _))),
            );

            solver.minimize(&(&buttons_pressed_a * cost_button_a + &buttons_pressed_b * cost_button_b));

            if z3::SatResult::Sat != solver.check(&[]) {
                return 0;
            };

            let model = solver.get_model().unwrap();
            let cost_ast = buttons_pressed_a * cost_button_a + buttons_pressed_b * cost_button_b;
            let final_cost = model.eval::<z3::ast::Int>(&cost_ast, true).unwrap().as_i64().unwrap();

            final_cost as u64
        }).sum()
}

fn part_1(input: &[u8]) -> u64 {
    solve(input, false)
}

fn part_2(input: &[u8]) -> u64 {
    solve(input, true)
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;

    const EXAMPLE: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE.as_bytes()), 480);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE.as_bytes()), 875318608908);
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
