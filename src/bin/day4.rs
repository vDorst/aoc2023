use aoc2023::{input_filename, read_input};

fn part1(data: &str) -> String {
    let lines = data.lines().filter(|s| !s.is_empty());

    let mut score = 0;

    for line in lines {
        let (_card, rest) = line.split_once(':').unwrap();
        let (lhs, rhs) = rest.split_once('|').unwrap();

        let left = lhs
            .split_whitespace()
            .map(|n| n.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        let right = rhs
            .split_whitespace()
            .filter_map(|n| {
                let rhs = n.parse::<u8>().unwrap();
                if left.iter().any(|n| *n == rhs) {
                    Some(rhs)
                } else {
                    None
                }
            })
            .collect::<Vec<u8>>();

        let sum = right.len();

        if sum != 0 {
            score += 2_usize.pow(sum as u32 - 1);
        }
    }
    score.to_string()
}

fn part2(data: &str) -> String {
    let lines = data.lines().filter(|s| !s.is_empty());

    let mut scores = Vec::<usize>::new();

    for line in lines {
        let (_card, rest) = line.split_once(':').unwrap();
        let (lhs, rhs) = rest.split_once('|').unwrap();

        let left = lhs
            .split_whitespace()
            .map(|n| n.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        let right = rhs
            .split_whitespace()
            .filter_map(|n| {
                let rhs = n.parse::<u8>().unwrap();
                if left.iter().any(|n| *n == rhs) {
                    Some(rhs)
                } else {
                    None
                }
            })
            .collect::<Vec<u8>>();

        scores.push(right.len());
    }

    let mut instances = vec![1_usize; scores.len()];

    scores.iter().enumerate().for_each(|(idx, n)| {
        let x = instances[idx];
        for idx in idx + 1..idx + n + 1 {
            if let Some(times) = instances.get_mut(idx) {
                *times += x;
            } else {
                break;
            }
        }
    });

    instances.iter().sum::<usize>().to_string()
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

    const SAMPLE: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn example_1() {
        assert_eq!(&part1(SAMPLE), "13");
    }
    #[test]
    fn example_2() {
        assert_eq!(&part2(SAMPLE), "30");
    }
}
