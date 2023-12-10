use aoc2023::{input_filename, read_input};
use winnow::{
    self,
    ascii::{alpha1, alphanumeric1, dec_int, digit1, line_ending},
    binary,
    combinator::{eof, opt, preceded, repeat, separated, terminated},
    token::{any, take, take_while},
    PResult, Parser,
};

#[derive(Debug, PartialEq, Eq)]
enum Pipe {
    Start,
    Hor,
    Ver,
    BendNE,
    BendNW,
    BendSE,
    BendSW,
    Ground,
}

impl Pipe {
    pub fn new(input: char) -> Option<Self> {
        match input {
            '.' => Some(Self::Ground),
            '|' => Some(Self::Ver),
            '-' => Some(Self::Hor),
            'L' => Some(Self::BendNE),
            'J' => Some(Self::BendNW),
            '7' => Some(Self::BendSW),
            'F' => Some(Self::BendSE),
            'S' => Some(Self::Start),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Game(Vec<Vec<Pipe>>);

fn parse_num(input: &mut &str) -> PResult<i32> {
    dec_int.parse_next(input)
}

fn parse_line(input: &mut &str) -> PResult<Vec<Pipe>> {
    let row = repeat(1.., any.verify_map(Pipe::new)).parse_next(input)?;
    line_ending.parse_next(input)?;

    Ok(row)
}

fn process(input: &str) -> PResult<Game> {
    let mut input = input;
    let input = &mut input;
    let grid = repeat(1.., parse_line).parse_next(input)?;
    // opt(line_ending).parse_next(input)?;
    // eof.parse_next(input)?;
    Ok(Game(grid))
}

#[derive(Debug)]
struct P {
    y: usize,
    x: usize,
}

impl P {
    pub fn new(x: usize, y: usize) -> P {
        Self { y, x }
    }
}

fn part1(input: &str) -> String {
    let g = process(input).unwrap();

    let mut start = P::new(0, 0);
    for (y, row) in g.0.iter().enumerate() {
        if let Some(x) = row.iter().position(|n| *n == Pipe::Start) {
            start = P::new(x, y);
            break;
        }
    }

    println!("Starpos: {start:?}");

    0.to_string()
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

    use crate::{parse_line, part1, part2, process, Game, Pipe};
    const SAMPLE: &str = r#"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
    
"#;

    const SAMPLE_2: &str = r#"-L|F7
7S-7|
L|7||
-L-J|
L|-JF
"#;

    #[test]
    fn process_data() {
        assert_eq!(
            parse_line.parse_peek("7-F7-\r\n"),
            Ok((
                "",
                vec![
                    Pipe::BendSW,
                    Pipe::Hor,
                    Pipe::BendSE,
                    Pipe::BendSW,
                    Pipe::Hor
                ]
            ))
        );

        let g = process(SAMPLE);

        assert_eq!(
            g,
            Ok(Game(vec![
                vec![
                    Pipe::BendSW,
                    Pipe::Hor,
                    Pipe::BendSE,
                    Pipe::BendSW,
                    Pipe::Hor
                ],
                vec![
                    Pipe::Ground,
                    Pipe::BendSE,
                    Pipe::BendNW,
                    Pipe::Ver,
                    Pipe::BendSW
                ],
                vec![
                    Pipe::Start,
                    Pipe::BendNW,
                    Pipe::BendNE,
                    Pipe::BendNE,
                    Pipe::BendSW
                ],
                vec![Pipe::Ver, Pipe::BendSE, Pipe::Hor, Pipe::Hor, Pipe::BendNW],
                vec![
                    Pipe::BendNE,
                    Pipe::BendNW,
                    Pipe::Ground,
                    Pipe::BendNE,
                    Pipe::BendNW
                ]
            ]))
        );

        let g = process(SAMPLE_2);
    }

    #[test]
    fn example_1() {
        assert_eq!(&part1(SAMPLE), "4");
    }

    #[test]
    fn example_1_s2() {
        assert_eq!(&part1(SAMPLE_2), "4");
    }
    // #[test]
    // fn example_2() {
    //     assert_eq!(&part2(SAMPLE), "2");
    // }
}
