use aoc2023::{input_filename, read_input};

#[derive(Debug)]

struct NumPos {
    y: u8,
    xs: u8,
    xe: u8,
}

#[derive(Debug)]
struct Num {
    value: u16,
    pos: NumPos,
}

#[derive(Debug)]
struct Sym {
    v: char,
    x: u8,
    y: u8,
    numbers: Vec<u16>,
}

fn part1(data: &str) -> String {
    let lines = data.lines().enumerate().filter(|(_y, s)| !s.is_empty());
    let mut obj: Option<Num> = None;
    let mut numbers = Vec::<Num>::new();
    let mut symbols = Vec::<Sym>::new();
    for (ly, line) in lines {
        let mut chars = line.char_indices();

        loop {
            let char = chars.next();
            match char {
                Some((_, '.')) | None => {
                    if obj.is_some() {
                        numbers.push(obj.take().unwrap());
                    }
                    if char.is_none() {
                        break;
                    }
                }
                Some((lx, c)) => {
                    if let Some(val) = c.to_digit(10) {
                        let val = val as u16;
                        if let Some(obj) = &mut obj {
                            obj.value *= 10;
                            obj.value += val;
                            obj.pos.xe = lx as u8;
                        } else {
                            obj = Some(Num {
                                value: val,
                                pos: NumPos {
                                    y: ly as u8,
                                    xs: lx as u8,
                                    xe: lx as u8,
                                },
                            })
                        }
                    } else {
                        if obj.is_some() {
                            numbers.push(obj.take().unwrap());
                        }
                        symbols.push(Sym {
                            v: c,
                            x: lx as u8,
                            y: ly as u8,
                            numbers: Vec::new(),
                        });
                    }
                }
            }
        }
    }

    // dbg!(numbers);
    // dbg!(symbols);

    numbers
        .iter()
        .filter_map(|num| {
            let xs = num.pos.xs.saturating_sub(1);
            let xe = num.pos.xe.saturating_add(1);
            let ys = num.pos.y.saturating_sub(1);
            let ye = num.pos.y.saturating_add(1);

            if symbols
                .iter()
                .any(|sym| sym.x >= xs && sym.x <= xe && sym.y >= ys && sym.y <= ye)
            {
                Some(u32::from(num.value))
            } else {
                None
            }
        })
        .sum::<u32>()
        .to_string()
}

fn part2(data: &str) -> String {
    let lines = data.lines().enumerate().filter(|(_y, s)| !s.is_empty());
    let mut obj: Option<Num> = None;
    let mut numbers = Vec::<Num>::new();
    let mut symbols = Vec::<Sym>::new();
    for (ly, line) in lines {
        let mut chars = line.char_indices();

        loop {
            let char = chars.next();
            match char {
                Some((_, '.')) | None => {
                    if obj.is_some() {
                        numbers.push(obj.take().unwrap());
                    }
                    if char.is_none() {
                        break;
                    }
                }
                Some((lx, c)) => {
                    if let Some(val) = c.to_digit(10) {
                        let val = val as u16;
                        if let Some(obj) = &mut obj {
                            obj.value *= 10;
                            obj.value += val;
                            obj.pos.xe = lx as u8;
                        } else {
                            obj = Some(Num {
                                value: val,
                                pos: NumPos {
                                    y: ly as u8,
                                    xs: lx as u8,
                                    xe: lx as u8,
                                },
                            })
                        }
                    } else {
                        if obj.is_some() {
                            numbers.push(obj.take().unwrap());
                        }
                        symbols.push(Sym {
                            v: c,
                            x: lx as u8,
                            y: ly as u8,
                            numbers: Vec::new(),
                        });
                    }
                }
            }
        }
    }

    // dbg!(numbers);
    // dbg!(symbols);

    numbers.iter().for_each(|num| {
        let xs = num.pos.xs.saturating_sub(1);
        let xe = num.pos.xe.saturating_add(1);
        let ys = num.pos.y.saturating_sub(1);
        let ye = num.pos.y.saturating_add(1);

        symbols
            .iter_mut()
            .filter(|sym| sym.v == '*')
            .for_each(|sym| {
                if sym.x >= xs && sym.x <= xe && sym.y >= ys && sym.y <= ye {
                    sym.numbers.push(num.value)
                }
            })
    });

    symbols
        .iter()
        .filter_map(|sym| {
            if sym.numbers.len() >= 2 {
                Some(sym.numbers.iter().map(|v| u32::from(*v)).product::<u32>())
            } else {
                None
            }
        })
        .sum::<u32>()
        .to_string()
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
    use crate::{part1, part2};

    const SAMPLE: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    #[test]
    fn example_1() {
        assert_eq!(&part1(SAMPLE), "4361");
    }
    #[test]
    fn example_2() {
        assert_eq!(&part2(SAMPLE), "467835");
    }
}
