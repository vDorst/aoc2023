use std::collections::BTreeMap;

use aoc2023::{input_filename, read_input};
use itertools::Itertools;
use num::Integer;
use winnow::{
    self,
    ascii::{alpha1, alphanumeric1, dec_int, digit1, line_ending},
    binary,
    combinator::{eof, opt, preceded, repeat, separated, terminated},
    token::take_while,
    PResult, Parser,
};

#[derive(Debug, PartialEq, Eq)]
struct Game {
    rows: Vec<Vec<i32>>,
}

fn search(input: &[i32]) -> i32 {
    let mut ret: Vec<Vec<i32>> = Vec::new();

    {
        let mut input: &[i32] = input;

        let mut count = 100;

        println!("input: {input:?}");

        loop {
            let diff = input
                .iter()
                .tuple_windows()
                .map(|(a, b)| b - a)
                .collect::<Vec<i32>>();

            let is_zero = diff.iter().all(|n| *n == 0);

            if is_zero {
                break;
            }

            println!(" next: {diff:?}");
            ret.push(diff);
            input = ret.last().unwrap();

            count -= 1;
            if count == 0 {
                panic!("Too many rounds!");
            }
        }
    }

    let mut add = 0;
    while let Some(row) = ret.pop() {
        add += *row.last().unwrap();
    }

    let ret = input.last().unwrap() + add;
    println!("ret {ret}");
    ret
}

fn parse_num(input: &mut &str) -> PResult<i32> {
    dec_int.parse_next(input)
}

fn parse_line(input: &mut &str) -> PResult<Vec<i32>> {
    let numbers = separated(1.., parse_num, " ").parse_next(input)?;
    line_ending.parse_next(input)?;

    Ok(numbers)
}

fn process(input: &str) -> PResult<Game> {
    let mut input = input;
    let input = &mut input;
    let nums = repeat(1.., parse_line).parse_next(input)?;
    // opt(line_ending).parse_next(input)?;
    // eof.parse_next(input)?;
    Ok(Game { rows: nums })
}

fn part1(input: &str) -> String {
    let g = process(input).unwrap();

    g.rows
        .iter()
        .map(|row| search(row))
        .sum::<i32>()
        .to_string()
}

fn part2(input: &str) -> String {
    let g = process(input).unwrap();

    0.to_string()
}

fn main() {
    let data = read_input(&format!("./input_{}.txt", input_filename(file!())));

    let numbers = part1(&data);
    println!("Part1: {numbers}");

    let numbers = part2(&data);
    println!("Part2: {numbers}");
}
#[cfg(test)]
mod tests {
    use winnow::Parser;

    use crate::{parse_line, part1, part2, process, Game};
    const SAMPLE: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"#;

    #[test]
    fn process_data() {
        assert_eq!(
            parse_line.parse_peek("0 3 6 9 12 15\r\n"),
            Ok(("", vec![0, 3, 6, 9, 12, 15]))
        );

        assert_eq!(
            parse_line.parse_peek("-1 2 4\r\n"),
            Ok(("", vec![-1, 2, 4]))
        );

        let g = process(SAMPLE);

        assert_eq!(
            g,
            Ok(Game {
                rows: vec![
                    vec![0, 3, 6, 9, 12, 15],
                    vec![1, 3, 6, 10, 15, 21],
                    vec![10, 13, 16, 21, 30, 45]
                ]
            })
        )
    }

    #[test]
    fn example_1() {
        assert_eq!(&part1(SAMPLE), "114");
    }
    // #[test]
    // fn example_2() {
    //     assert_eq!(&part2(SAMPLE_2), "6");
    // }
}
