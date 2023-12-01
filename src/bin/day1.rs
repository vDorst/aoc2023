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

fn part2(data: &str) -> usize {
    let data = data.as_bytes();

    let mut total: i16 = 0;

    for (pos, byte) in data.iter().enumerate() {
        match *byte {
            b'(' => total += 1,
            b')' => total -= 1,
            _ => (),
        }
        if total == -1 {
            return pos + 1;
        }
    }
    0
}

fn main() {
    let data = read_input(&format!("./input_{}.txt", input_filename(file!())));

    let numbers = part1(&data);
    println!("Part1: {numbers}");

    let pos = part2(&data);
    println!("Part2: pos = {pos}");
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const SAMPLES: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

    #[test]
    fn example_1() {
        assert_eq!(part1(SAMPLES), 142);
    }

    // #[test]
    // fn example_2() {
    //     let pos = part2(")");
    //     assert_eq!(pos, 1);

    //     let pos = part2("()())");
    //     assert_eq!(pos, 5);
    // }
}
