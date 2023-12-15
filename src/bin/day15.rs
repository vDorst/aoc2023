use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use aoc2023::{input_filename, read_input};
use winnow::{
    self,
    ascii::{alpha1, alphanumeric1, dec_int, digit1, line_ending},
    binary,
    combinator::{alt, eof, opt, preceded, repeat, separated, terminated},
    token::{any, take, take_till1, take_while},
    PResult, Parser,
};

fn parse_line(input: &mut &str) -> PResult<Vec<u8>> {
    let row = separated(
        1..,
        take_till1(|c| c == ',' || c == '\n' || c == '\r').map(|s: &str| {
            s.as_bytes()
                .iter()
                .fold(0_u8, |acc, c| acc.wrapping_add(*c).wrapping_mul(17))
        }),
        ',',
    )
    .parse_next(input)?;
    line_ending.parse_next(input)?;

    Ok(row)
}

fn process(input: &str) -> PResult<Vec<u8>> {
    let mut input = input;
    let input = &mut input;
    let hash = parse_line.parse_next(input)?;
    opt(line_ending).parse_next(input)?;
    eof.parse_next(input)?;
    Ok(hash)
}

fn part1(input: &str) -> String {
    let g = process(input).unwrap();

    g.iter()
        .fold(0_usize, |acc, x| acc + usize::from(*x))
        .to_string()
}

fn part2(input: &str) -> String {
    let mut g = process(input).unwrap();
    0.to_string()
}

fn main() {
    let data = read_input(&format!("./input_{}.txt", input_filename(file!())));

    let start = Instant::now();
    let numbers = part1(&data);
    println!("Part1: {numbers}, {} uS", start.elapsed().as_micros());

    let start = Instant::now();
    let numbers = part2(&data);
    println!("Part2: {numbers}, {} uS", start.elapsed().as_micros());
}
#[cfg(test)]
mod tests {
    use winnow::Parser;

    use crate::{parse_line, part1, part2, process};
    const SAMPLE: &str = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
"#;
    #[test]
    fn process_data() {
        assert_eq!(
            parse_line.parse_peek(SAMPLE),
            Ok(("", vec![30, 253, 97, 47, 14, 180, 9, 197, 48, 214, 231]))
        );

        let g = process(SAMPLE).unwrap();
    }

    #[test]
    fn example_1() {
        assert_eq!(&part1(SAMPLE), "1320");
    }

    // #[test]
    // fn example_2() {
    //     assert_eq!(&part2(SAMPLE), "64");
    // }
}
