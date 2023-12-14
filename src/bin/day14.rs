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
    token::{any, take, take_while},
    PResult, Parser,
};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Game(Vec<Vec<I>>);

impl Game {
    pub fn show_map(&self) {
        println!("MAP:");
        for row in &self.0 {
            let mut s = String::new();
            for tile in row {
                s.push(match tile {
                    I::E => '.',
                    I::R => 'O',
                    I::C => '#',
                });
            }
            println!("{s}");
        }
    }

    pub fn move_north(&mut self) {
        for y in 1..self.0.len() {
            for x in 0..self.0[0].len() {
                if matches!(self.0[y][x], I::R) {
                    let mut found: Option<usize> = None;
                    for ys in (0..y).rev() {
                        if matches!(self.0[ys][x], I::E) {
                            found = Some(ys);
                        } else {
                            break;
                        }
                    }
                    if let Some(ys) = found {
                        self.0[y][x] = I::E;
                        self.0[ys][x] = I::R;
                    }
                }
            }
        }
    }

    pub fn move_south(&mut self) {
        for y in (0..self.0.len() - 1).rev() {
            for x in 0..self.0[0].len() {
                if matches!(self.0[y][x], I::R) {
                    let mut found: Option<usize> = None;
                    for ys in y + 1..self.0.len() {
                        if matches!(self.0[ys][x], I::E) {
                            found = Some(ys);
                        } else {
                            break;
                        }
                    }
                    if let Some(ys) = found {
                        self.0[y][x] = I::E;
                        self.0[ys][x] = I::R;
                    }
                }
            }
        }
    }

    pub fn move_east(&mut self) {
        for y in 0..self.0.len() {
            let x_len = self.0[0].len();
            for x in (0..x_len - 1).rev() {
                if matches!(self.0[y][x], I::R) {
                    let mut found: Option<usize> = None;
                    for xs in x + 1..x_len {
                        if matches!(self.0[y][xs], I::E) {
                            found = Some(xs);
                        } else {
                            break;
                        }
                    }
                    if let Some(xs) = found {
                        self.0[y][x] = I::E;
                        self.0[y][xs] = I::R;
                    }
                }
            }
        }
    }

    pub fn move_west(&mut self) {
        for y in 0..self.0.len() {
            let x_len = self.0[0].len();
            for x in 1..x_len {
                if matches!(self.0[y][x], I::R) {
                    let mut found: Option<usize> = None;
                    for xs in (0..x).rev() {
                        if matches!(self.0[y][xs], I::E) {
                            found = Some(xs);
                        } else {
                            break;
                        }
                    }
                    if let Some(xs) = found {
                        self.0[y][x] = I::E;
                        self.0[y][xs] = I::R;
                    }
                }
            }
        }
    }

    pub fn score(&self) -> usize {
        let rows = self.0.len();

        self.0
            .iter()
            .enumerate()
            .map(|(idx, row)| row.iter().filter(|tile| **tile == I::R).count() * (rows - idx))
            .sum::<usize>()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum I {
    /// empty spaces (.)
    E,
    /// single rounded rock (O)
    R,
    /// Cube-shaped rocks (#) don't contribute to load.)
    C,
}

impl I {
    fn new(item: char) -> Option<Self> {
        match item {
            '#' => Some(Self::C),
            'O' => Some(Self::R),
            '.' => Some(Self::E),
            _ => None,
        }
    }
}

fn parse_line(input: &mut &str) -> PResult<Vec<I>> {
    let row = repeat(1.., any.verify_map(|c: char| I::new(c))).parse_next(input)?;
    line_ending.parse_next(input)?;

    Ok(row)
}

fn process(input: &str) -> PResult<Game> {
    let mut input = input;
    let input = &mut input;
    let grid = repeat(1.., parse_line).parse_next(input)?;
    opt(line_ending).parse_next(input)?;
    eof.parse_next(input)?;
    Ok(Game(grid))
}

fn part1(input: &str) -> String {
    let mut g = process(input).unwrap();

    // g.show_map();

    g.move_north();

    // g.show_map();

    g.score().to_string()
}

fn part2(input: &str) -> String {
    let mut g = process(input).unwrap();

    // g.show_map();

    // return "0".to_string();

    let mut test_cnt = 1000;

    for _ in 0..test_cnt {
        g.move_north();
        g.move_west();
        g.move_south();
        g.move_east();
    }

    let mut gs = g.clone();
    let mut repeat_cnt = 0;
    let mut tested = None;
    'l: loop {
        {
            g.move_north();
            g.move_west();
            g.move_south();
            g.move_east();
            test_cnt += 1;
            repeat_cnt += 1;
        }

        if let Some(end) = tested {
            if test_cnt == end {
                if gs == g {
                    println!("Cycle found again");
                    break;
                } else {
                    println!("No found");
                    gs = g.clone();
                    tested = None;
                    repeat_cnt = 0;
                }
            }
        } else if gs == g {
            println!("Cache hit after {test_cnt} len {}", repeat_cnt);
            tested = Some(test_cnt + repeat_cnt);
        }
    }

    let times = (1000000000 - test_cnt) % repeat_cnt;

    // println!("last part {times}");

    for n in 0..times {
        g.move_north();
        g.move_west();
        g.move_south();
        g.move_east();
        // println!("Score: {n}, {}", g.score());
    }

    g.score().to_string()
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

    use crate::{parse_line, part1, part2, process, Game, I};
    const SAMPLE: &str = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....

"#;
    #[test]
    fn process_data() {
        assert_eq!(
            parse_line.parse_peek("O.#.\n"),
            Ok(("", vec![I::R, I::E, I::C, I::E]))
        );

        let g = process(SAMPLE).unwrap();
    }

    #[test]
    fn example_1() {
        assert_eq!(&part1(SAMPLE), "136");
    }

    #[test]
    fn example_2() {
        assert_eq!(&part2(SAMPLE), "64");
    }
}
