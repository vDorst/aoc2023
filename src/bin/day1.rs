use aoc2023::{input_filename, read_input};

fn part1(data: &str) -> usize {
    data.as_bytes()
        .split(|c| *c == b'\n')
        .filter(|s| !s.is_empty())
        .map(|c| {
            let numbers = c
                .iter()
                .map(|c| (*c).wrapping_sub(b'0'))
                .filter(|c| *c < 10)
                //.filter(|c| (b'0'..=b'9').contains(*c))
                .collect::<Vec<u8>>();

            let first = numbers.first().unwrap();
            let last = numbers.last().unwrap();
            *first * 10 + *last
        })
        .fold(0_usize, |acc, n| acc + usize::from(n))
}

const NUMBERS: [&[u8]; 9] = [
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

fn part2(data: &str) -> usize {
    data.as_bytes()
        .split(|c| *c == b'\n')
        .filter(|s| !s.is_empty())
        .map(|c| {
            let mut numbers = c
                .iter()
                .enumerate()
                .filter_map(|(pos, c)| {
                    let n = (*c).wrapping_sub(b'0');
                    if n < 10 {
                        Some((pos as u8, n))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(u8, u8)>>();

            let mut pos = 0;
            loop {
                for (s, n) in NUMBERS.iter().zip(1_u8..) {
                    if c[pos..].starts_with(s) {
                        numbers.push((pos as u8, n));
                        pos += s.len() - 2;
                        break;
                    }
                }
                pos += 1;
                if pos >= c.len() - 2 {
                    break;
                }
            }

            numbers.sort_by(|a, b| a.0.cmp(&b.0));

            let first = numbers.first().unwrap().1;
            let last = numbers.last().unwrap().1;
            first * 10 + last
        })
        .fold(0_usize, |acc, n| acc + usize::from(n))
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

    const SAMPLE: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

    const SAMPLE2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn example_1() {
        assert_eq!(part1(SAMPLE), 142);
    }

    #[test]
    fn example_2() {
        assert_eq!(part2(SAMPLE2), 281);
    }
}
