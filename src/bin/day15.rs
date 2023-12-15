use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use aoc2023::{input_filename, read_input};
use winnow::{
    self,
    ascii::{alpha1, alphanumeric1, dec_int, dec_uint, digit1, line_ending},
    binary,
    combinator::{alt, eof, opt, preceded, repeat, separated, terminated},
    token::{any, take, take_till1, take_while},
    PResult, Parser,
};

fn hashes(input: Vec<&str>) -> Vec<u8> {
    input.into_iter().map(hash).collect()
}

fn hash(input: &str) -> u8 {
    input
        .as_bytes()
        .iter()
        .fold(0_u8, |acc, c| acc.wrapping_add(*c).wrapping_mul(17))
}

fn parse_line<'a>(input: &mut &'a str) -> PResult<Vec<&'a str>> {
    let row = separated(1.., take_till1(|c| c == ',' || c == '\n' || c == '\r'), ',')
        .parse_next(input)?;
    line_ending.parse_next(input)?;

    Ok(row)
}

fn process(input: &str) -> PResult<Vec<&str>> {
    let mut input = input;
    let input = &mut input;
    let hash = parse_line.parse_next(input)?;
    opt(line_ending).parse_next(input)?;
    eof.parse_next(input)?;
    Ok(hash)
}

fn part1(input: &str) -> String {
    let g = process(input).unwrap();

    hashes(g)
        .iter()
        .fold(0_usize, |acc, x| acc + usize::from(*x))
        .to_string()
}

#[derive(Debug, PartialEq)]
enum Act<'a> {
    Del(&'a str),
    Add(&'a str, u8),
}

fn parse_box<'a>(input: &'a str) -> PResult<Act<'a>> {
    let (_, act) = alt((
        terminated(alpha1, '-').map(|begin: &str| Act::Del(begin)),
        (terminated(alpha1, "="), dec_uint).map(|(begin, lens)| Act::Add(begin, lens)),
    ))
    .parse_peek(input)?;
    Ok(act)
}

fn part2(input: &str) -> String {
    let mut g = process(input).unwrap();

    const ARRAY_REPEAT_VALUE: Vec<(&str, u8)> = Vec::<(&str, u8)>::new();
    let mut boxes = [ARRAY_REPEAT_VALUE; 256];

    for item in g {
        match parse_box(item).unwrap() {
            Act::Del(hash_str) => {
                let box_idx = usize::from(hash(hash_str));
                boxes[box_idx].retain(|s| s.0 != hash_str);
            }

            Act::Add(hash_str, lens) => {
                let box_idx = usize::from(hash(hash_str));
                if let Some(b) = boxes[box_idx].iter_mut().find(|s| s.0 == hash_str) {
                    b.1 = lens;
                } else {
                    boxes[box_idx].push((hash_str, lens));
                }
            }
        }
    }

    let mut total = 0;
    for (box_idx, b) in boxes.iter().enumerate() {
        if b.is_empty() {
            continue;
        }
        let mut s = String::new();
        for (slot_idx, (item, lens)) in b.iter().enumerate() {
            s.push_str(&format!(" [{} {}]", item, lens));
            total += (box_idx + 1) * (slot_idx + 1) * usize::from(*lens);
        }

        println!("Box {box_idx}: {s}");
    }

    total.to_string()
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

    use crate::{hashes, parse_box, parse_line, part1, part2, process, Act};
    const SAMPLE: &str = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
"#;
    #[test]
    fn process_data() {
        let data = parse_line.parse_peek(SAMPLE);
        assert_eq!(
            data,
            Ok((
                "",
                vec![
                    "rn=1", "cm-", "qp=3", "cm=2", "qp-", "pc=4", "ot=9", "ab=5", "pc-", "pc=6",
                    "ot=7"
                ]
            ))
        );

        assert_eq!(
            hashes(data.unwrap().1),
            vec![30_u8, 253, 97, 47, 14, 180, 9, 197, 48, 214, 231]
        );

        assert_eq!(parse_box("rn=1"), Ok(Act::Add("rn", 1)));
        assert_eq!(parse_box("cm-"), Ok(Act::Del("cm")));
    }

    #[test]
    fn example_1() {
        assert_eq!(&part1(SAMPLE), "1320");
    }

    #[test]
    fn example_2() {
        assert_eq!(&part2(SAMPLE), "145");
    }
}
