use aoc2023::{input_filename, read_input};
use winnow::{
    ascii::digit1,
    combinator::{opt, repeat, terminated},
    token::{tag, take_until1},
    PResult, Parser,
};

#[derive(Debug)]
struct R {
    des: u64,
    src: u64,
    len: u64,
}

struct Game {
    seeds: Vec<u64>,
    soil: Vec<(String, Vec<R>)>,
}

impl Game {
    fn find(&self, seed: u64) -> u64 {
        let mut num = seed;
        for (name, items) in self.soil.iter() {
            if let Some(r) = items.iter().find(|r| (r.src..r.src + r.len).contains(&num)) {
                let p = num - r.src + r.des;
                // println!("{name}: found {num} in {r:?}: {p}");
                // num = num - r.src + r.des;
                num = p;
            } else {
                // println!("{name}: Nomapping {num}");
                // return None;
            }
        }

        num
    }
}

fn parse_digits(input: &mut &str) -> PResult<u64> {
    digit1.parse_to().parse_next(input)
}

fn nonempty<'a>(input: &mut &'a str) -> PResult<&'a str> {
    let ident = take_until1("\n").parse_next(input)?;
    let _ = tag("\n").parse_next(input)?;
    Ok(ident)
}

fn parse_block(input: &mut &str) -> PResult<(String, Vec<R>)> {
    let name = nonempty.parse_next(input)?.to_string();
    let idents: Vec<&str> = repeat(1.., nonempty).parse_next(input)?;
    let a = idents.iter().map(|s| parse_numbers(s).unwrap()).collect();
    // let a = Vec::new();
    let _ = tag("\n").parse_next(input)?;
    Ok((name, a))
}

fn parse_list(input: &mut &str) -> PResult<Vec<u64>> {
    let mut list = Vec::new();
    while let Some(output) = opt(terminated(parse_digits, opt(' '))).parse_next(input)? {
        list.push(output)
    }
    Ok(list)
}

fn parse_numbers(input: &str) -> PResult<R> {
    let numbers = parse_list.parse_peek(input)?.1;
    let a = R {
        des: numbers[0],
        src: numbers[1],
        len: numbers[2],
    };
    Ok(a)
}

fn parse_seeds(input: &mut &str) -> PResult<Vec<u64>> {
    "seeds: ".parse_next(input)?;
    let seeds = parse_list(input)?;
    repeat(2.., tag("\n")).parse_next(input)?;
    Ok(seeds)
}

fn process(mut input: &str) -> Game {
    let seeds = parse_seeds.parse_next(&mut input).unwrap();

    let mut soil = Vec::new();
    loop {
        let Ok(item) = parse_block.parse_next(&mut input) else {
            break;
        };
        soil.push(item);
    }

    assert_eq!(soil.len(), 7);

    Game {
        seeds,
        soil: dbg!(soil),
    }
}

fn part1(input: &str) -> String {
    let g = process(input);

    g.seeds
        .iter()
        .map(|n| g.find(*n))
        .min()
        .unwrap()
        .to_string()
}

fn part2(input: &str) -> String {
    let g = process(input);

    let mut ans = u64::MAX;
    for seeds in g.seeds.chunks_exact(2) {
        let m = (seeds[0]..seeds[0] + seeds[1])
            .map(|n| g.find(n))
            .min()
            .unwrap();
        ans = ans.min(m);
    }

    ans.to_string()
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
    use crate::{part1, part2, process};

    const SAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4

";

    #[test]
    fn example_p1() {
        let g = process(SAMPLE);

        assert_eq!(g.find(79), 82);
        assert_eq!(g.find(14), 43);
        assert_eq!(g.find(55), 86);
        assert_eq!(g.find(13), 35);
    }

    #[test]
    fn example_1() {
        assert_eq!(&part1(SAMPLE), "35");
    }
    #[test]
    fn example_2() {
        assert_eq!(&part2(SAMPLE), "46");
    }
}
