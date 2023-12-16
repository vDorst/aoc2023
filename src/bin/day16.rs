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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    /// '.'
    Empty,
    /// '-'
    SplitHor,
    /// '|'
    SplitVer,
    /// '/'
    Slash,
    /// '\\'
    BackSlash,
}

impl Tile {
    pub fn new(input: char) -> Option<Self> {
        match input {
            '.' => Some(Self::Empty),
            '|' => Some(Self::SplitVer),
            '-' => Some(Self::SplitHor),
            '/' => Some(Self::Slash),
            '\\' => Some(Self::BackSlash),
            _ => None,
        }
    }
}

#[repr(packed)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct TileInfo {
    tile: Tile,
    energized: bool,
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    grid: Vec<Vec<TileInfo>>,
    backtrack: Vec<(P, Dir)>,
    dir: Dir,
    pos: P,
}
impl Game {
    pub fn get(&self, pos: P) -> TileInfo {
        self.grid[pos.y][pos.x]
    }

    pub fn mark(&mut self) {
        let pos = self.pos;
        self.grid[pos.y][pos.x].energized = true;
    }

    pub fn next(&mut self) -> bool {
        self.mark();
        let p = self.pos;
        let tileinfo = self.get(p);

        let dir = self.dir;

        let dir = match tileinfo.tile {
            // '/'
            Tile::Slash => match dir {
                Dir::Up => Dir::Right,
                Dir::Down => Dir::Left,
                Dir::Left => Dir::Down,
                Dir::Right => Dir::Up,
            },
            // '\'
            Tile::BackSlash => match dir {
                Dir::Up => Dir::Left,
                Dir::Down => Dir::Right,
                Dir::Left => Dir::Up,
                Dir::Right => Dir::Down,
            },
            // '-'
            Tile::SplitHor => {
                match dir {
                    Dir::Up | Dir::Down => {
                        // Add this tile for backtrack
                        self.backtrack.push((p, Dir::Right));
                        // Always go left first
                        Dir::Left
                    }
                    e => e,
                }
            }
            // '|'
            Tile::SplitVer => {
                match dir {
                    Dir::Left | Dir::Right => {
                        // Add this tile for backtrack
                        self.backtrack.push((p, Dir::Down));
                        // Always go left first
                        Dir::Up
                    }
                    e => e,
                }
            }
            Tile::Empty => dir,
        };

        let dead_end = if let Some(p) = self.search(dir) {
            let ti = self.get(p);
            if matches!(ti.tile, Tile::SplitHor | Tile::SplitVer) && ti.energized {
                true
            } else {
                self.dir = dir;
                self.pos = p;
                false
            }
        } else {
            true
        };

        println!("DE: {dead_end}, P {:?} D {:?}", self.pos, self.dir);

        if dead_end {
            if let Some((new_pos, new_dir)) = self.backtrack.pop() {
                self.pos = new_pos;
                self.dir = new_dir;
                println!("BackTrack: P {:?} D {:?}", self.pos, self.dir);
            } else {
                return false;
            }
        }

        true
    }

    pub fn show(&self) {
        println!("MAP");
        for row in &self.grid {
            let mut s = String::new();

            for tileinfo in row {
                s.push_str(
                    &(if tileinfo.energized {
                        format!("\x1b[42m")
                    } else {
                        format!("\x1b[0m")
                    }),
                );
                s.push(match tileinfo.tile {
                    Tile::Empty => '.',
                    Tile::SplitHor => '-',
                    Tile::SplitVer => '|',
                    Tile::Slash => '/',
                    Tile::BackSlash => '\\',
                })
            }
            println!("{s}\x1b[0m");
        }
    }

    pub fn search(&self, dir_to: Dir) -> Option<P> {
        let mut pos = self.pos;
        // println!("SEARCH() {pos:?}, dir {:?}", dir_from);
        match dir_to {
            Dir::Up => {
                if let Some(y) = pos.y.checked_sub(1) {
                    pos.y = y;
                    return Some(pos);
                }
            }
            Dir::Down => {
                if self.pos.y < self.grid.len() - 1 {
                    pos.y += 1;
                    return Some(pos);
                }
            }
            Dir::Left => {
                if let Some(x) = pos.x.checked_sub(1) {
                    pos.x = x;
                    return Some(pos);
                }
            }
            Dir::Right => {
                if pos.x < self.grid[0].len() - 1 {
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

fn parse_line(input: &mut &str) -> PResult<Vec<TileInfo>> {
    let row = repeat(
        1..,
        any.verify_map(Tile::new).map(|tile: Tile| TileInfo {
            tile,
            energized: false,
        }),
    )
    .parse_next(input)?;
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
        backtrack: Vec::new(),
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

    g.pos = P { y: 0, x: 0 };
    g.dir = Dir::Right;

    let mut cnt = 0;
    while g.next() {
        cnt += 1;
        if cnt > 10000 {
            println!("LIMIT!!!!");
            break;
        }
    }
    g.show();

    g.grid
        .iter()
        .map(|tile| tile.iter().filter(|tile| tile.energized).count())
        .sum::<usize>()
        .to_string()
}

fn part2(input: &str) -> String {
    let mut g = process(input).unwrap();

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

    use crate::{parse_line, part1, part2, process, Dir, Game, Tile, TileInfo, P};
    const SAMPLE: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
    
"#;

    #[test]
    fn process_data() {
        assert_eq!(
            parse_line.parse_peek("|.-.\\/\r\n"),
            Ok((
                "",
                vec![
                    TileInfo {
                        tile: Tile::SplitVer,
                        energized: false
                    },
                    TileInfo {
                        tile: Tile::Empty,
                        energized: false
                    },
                    TileInfo {
                        tile: Tile::SplitHor,
                        energized: false
                    },
                    TileInfo {
                        tile: Tile::Empty,
                        energized: false
                    },
                    TileInfo {
                        tile: Tile::BackSlash,
                        energized: false
                    },
                    TileInfo {
                        tile: Tile::Slash,
                        energized: false
                    },
                ]
            ))
        );
    }

    #[test]
    fn example_1() {
        assert_eq!(&part1(SAMPLE), "46");
    }

    // #[test]
    // fn example_1_s2() {
    //     assert_eq!(&part1(SAMPLE_2), "8");
    // }
    // #[test]
    // fn example_2() {
    //     assert_eq!(&part2(SAMPLE_P2), "10");
    // }
}
