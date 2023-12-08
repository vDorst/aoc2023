use std::collections::BTreeMap;

use aoc2023::{input_filename, read_input};
use num::Integer;
use winnow::{
    self,
    ascii::{alphanumeric1, line_ending},
    combinator::{repeat, terminated},
    token::take_while,
    PResult, Parser,
};

#[derive(Debug, PartialEq, Eq)]
struct Game<'a> {
    dir: &'a str,
    elements: BTreeMap<u32, (u32, u32)>,
}

impl<'a> Game<'a> {
    fn search(&self, what: &str) -> usize {
        let tag = str_to_u32(what);
        if !self.elements.contains_key(&tag) {
            panic!("Could not found {what}");
        }

        let mut dir = self.dir.as_bytes().iter().cycle();

        let mut search_tag: u32 = str_to_u32("AAA");

        let mut counter = 0;
        loop {
            counter += 1;
            let tags = self.elements.get(&search_tag).unwrap();
            let d = dir.next().unwrap();
            search_tag = if *d == b'L' { tags.0 } else { tags.1 };

            if search_tag == str_to_u32("ZZZ") {
                break;
            }
        }

        counter
    }

    fn search_p2(&self) -> usize {
        let search_tags: Vec<u32> = self
            .elements
            .keys()
            .filter(|num| (**num & 0xFF) as u8 == b'A')
            .copied()
            .collect();

        let mut cnts = Vec::new();

        for mut search_tag in search_tags {
            let mut dir = self.dir.as_bytes().iter().cycle();
            let mut counter: usize = 0;
            loop {
                counter += 1;
                let go_left = *dir.next().unwrap() == b'L';

                let tags = self.elements.get(&search_tag).unwrap();
                search_tag = if go_left { tags.0 } else { tags.1 };

                if (search_tag & 0xFF) as u8 == b'Z' {
                    //println!("count: {counter}");
                    cnts.push(counter);
                    break;
                }
            }
        }

        let start = self.dir.len();
        cnts.iter().fold(start, |acc, num| acc.lcm(num))
    }
}

fn str_to_u32(input: &str) -> u32 {
    input
        .as_bytes()
        .iter()
        .copied()
        .fold(0_u32, |acc, c| (acc << 8) + u32::from(c))
}

fn parse_line(input: &mut &str) -> PResult<(u32, (u32, u32))> {
    let lbl = alphanumeric1.parse_next(input)?;
    " = (".parse_next(input)?;
    let a = alphanumeric1.parse_next(input)?;
    ", ".parse_next(input)?;
    let b = alphanumeric1.parse_next(input)?;
    ")".parse_next(input)?;
    Ok((str_to_u32(lbl), (str_to_u32(a), str_to_u32(b))))
}

fn process(input: &str) -> PResult<Game<'_>> {
    let mut input = input;
    let input = &mut input;
    let dir = terminated(take_while(1.., [b'R', b'L']), line_ending).parse_next(input)?;
    line_ending.parse_next(input)?;
    let ele: BTreeMap<u32, (u32, u32)> =
        repeat(1.., terminated(parse_line, line_ending)).parse_next(input)?;
    Ok(Game { dir, elements: ele })
}

fn part1(input: &str) -> String {
    let g = process(input).unwrap();

    let num = g.search("AAA");

    num.to_string()
}

fn part2(input: &str) -> String {
    let g = process(input).unwrap();

    let num = g.search_p2();

    num.to_string()
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
    use crate::{part1, part2, process, str_to_u32};

    const SAMPLE: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

    const SAMPLE_2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

    #[test]
    fn process_data() {
        let g = process(SAMPLE);
        assert!(g.is_ok());
        let g = g.unwrap();
        assert_eq!(g.dir, "RL");
        assert_eq!(g.elements.len(), 7);
        assert_eq!(
            g.elements.get(&str_to_u32("AAA")),
            Some(&(str_to_u32("BBB"), str_to_u32("CCC")))
        );
        assert_eq!(
            g.elements.get(&str_to_u32("ZZZ")),
            Some(&(str_to_u32("ZZZ"), str_to_u32("ZZZ")))
        );
    }

    #[test]
    fn process_data_p2() {
        let g = process(SAMPLE_2);
        assert!(g.is_ok());
        let g = g.unwrap();
        assert_eq!(g.dir, "LR");
        assert_eq!(g.elements.len(), 8);
        assert_eq!(
            g.elements.get(&str_to_u32("11A")),
            Some(&(str_to_u32("11B"), str_to_u32("XXX")))
        );
        assert_eq!(
            g.elements.get(&str_to_u32("22Z")),
            Some(&(str_to_u32("22B"), str_to_u32("22B")))
        );
    }

    #[test]
    fn example_1() {
        assert_eq!(&part1(SAMPLE), "2");
    }
    #[test]
    fn example_2() {
        assert_eq!(&part2(SAMPLE_2), "6");
    }
}
