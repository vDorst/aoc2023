use std::ops::Not;

use aoc2023::{input_filename, read_input};
use winnow::{
    self,
    ascii::{alpha1, alphanumeric1, dec_int, digit1, line_ending},
    binary,
    combinator::{eof, opt, preceded, repeat, separated, terminated},
    token::{any, take, take_while},
    PResult, Parser,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Not for Dir {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Dir::Up => Self::Down,
            Dir::Down => Self::Up,
            Dir::Left => Self::Right,
            Dir::Right => Self::Left,
        }
    }
}

impl Dir {
    pub fn next(self, pipe: Pipe) -> Option<Self> {
        // println!("Dir: NEXT: {pipe:?}, {:?}", self);
        // Direction From -> Direction Going
        match pipe {
            Pipe::Ver => {
                if self == Self::Up {
                    Some(Self::Down)
                } else if self == Self::Down {
                    Some(Self::Up)
                } else {
                    None
                }
            }
            Pipe::Hor => {
                if self == Self::Left {
                    Some(Self::Right)
                } else if self == Self::Right {
                    Some(Self::Left)
                } else {
                    None
                }
            }
            Pipe::BendNE => {
                if self == Self::Up {
                    Some(Self::Right)
                } else if self == Self::Right {
                    Some(Self::Up)
                } else {
                    None
                }
            }
            Pipe::BendNW => {
                if self == Self::Up {
                    Some(Self::Left)
                } else if self == Self::Left {
                    Some(Self::Up)
                } else {
                    None
                }
            }
            Pipe::BendSE => {
                if self == Self::Down {
                    Some(Self::Right)
                } else if self == Self::Right {
                    Some(Self::Down)
                } else {
                    None
                }
            }
            Pipe::BendSW => {
                if self == Self::Down {
                    Some(Self::Left)
                } else if self == Self::Left {
                    Some(Self::Down)
                } else {
                    None
                }
            }
            Pipe::Ground | Pipe::Start => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
struct Game {
    grid: Vec<Vec<Pipe>>,
    pos: P,
    dir: Dir,
    cnt: usize,
}

impl Game {
    pub fn get(&self, pos: P) -> Pipe {
        self.grid[pos.y][pos.x]
    }

    pub fn next(&mut self) -> bool {
        let dir = !self.dir;
        if let Some(p) = self.search(dir) {
            let pipe = self.get(p);
            // println!("NEXT: {p:?} D {:?} {pipe:?}", self.dir);
            if let Some(l) = dir.next(pipe) {
                self.dir = l;
                self.pos = p;
                self.cnt += 1;
                return true;
            }
        }

        false
    }

    pub fn find_start(&mut self) -> bool {
        for (y, row) in self.grid.iter().enumerate() {
            if let Some(x) = row.iter().position(|n| *n == Pipe::Start) {
                self.pos = P::new(x, y);
                break;
            }
        }

        for dir in [Dir::Down, Dir::Up, Dir::Left, Dir::Right] {
            self.dir = dir;
            if self.next() {
                return true;
            }
        }

        false
    }

    pub fn search(&self, dir_from: Dir) -> Option<P> {
        let mut pos = self.pos;
        // println!("SEARCH() {pos:?}, dir {:?}", dir_from);
        match dir_from {
            Dir::Down => {
                if let Some(y) = pos.y.checked_sub(1) {
                    pos.y = y;
                    return Some(pos);
                }
            }
            Dir::Up => {
                if self.pos.y < self.grid.len() {
                    pos.y += 1;
                    return Some(pos);
                }
            }
            Dir::Right => {
                if let Some(x) = pos.x.checked_sub(1) {
                    pos.x = x;
                    return Some(pos);
                }
            }
            Dir::Left => {
                if pos.x < self.grid[0].len() {
                    pos.x += 1;
                    return Some(pos);
                }
            }
        }
        None
    }
}

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
    Ok(Game {
        grid,
        pos: P { y: 0, x: 0 },
        dir: Dir::Down,
        cnt: 0,
    })
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
    let mut g = process(input).unwrap();

    let start_found = g.find_start();

    println!("Starpos: P {:?} D {:?}", g.pos, g.dir);

    while g.next() {
        if g.cnt > 140 * 140 {
            println!("LIMIT!!!!");
            break;
        }
    }

    ((g.cnt + 1) / 2).to_string()
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

    use crate::{parse_line, part1, part2, process, Dir, Game, Pipe, P};
    const SAMPLE: &str = r#".....
.S-7.
.|.|.
.L-J.
.....
    
"#;

    const SAMPLE_2: &str = r#"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
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

        let mut g = process(SAMPLE_2);

        assert_eq!(
            g,
            Ok(Game {
                grid: vec![
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
                ],
                pos: P { y: 0, x: 0 },
                dir: crate::Dir::Down,
                cnt: 0,
            })
        );

        let g = process(SAMPLE_2);

        let mut g = g.unwrap();

        assert!(g.find_start());
        assert_eq!(g.pos, P::new(0, 3));
        assert_eq!(g.get(g.pos), Pipe::Ver);
        assert_eq!(g.dir, Dir::Down);

        assert!(g.next());
        assert_eq!(g.dir, Dir::Right);
        assert_eq!(g.pos, P::new(0, 4));

        assert!(g.next());
        assert_eq!(g.dir, Dir::Up);
        assert_eq!(g.pos, P::new(1, 4));
        assert_eq!(g.cnt, 3);
    }

    #[test]
    fn example_1() {
        assert_eq!(&part1(SAMPLE), "4");
    }

    #[test]
    fn example_1_s2() {
        assert_eq!(&part1(SAMPLE_2), "8");
    }
    // #[test]
    // fn example_2() {
    //     assert_eq!(&part2(SAMPLE), "2");
    // }
}
