use aoc2023::{input_filename, read_input};

fn part1(input: &[R]) -> String {
    input
        .iter()
        .map(|r| {
            (0..r.t)
                .filter(|hold| {
                    let time = r.t - hold;
                    let dis = time * hold;
                    dis > r.d
                })
                .count()
        })
        .product::<usize>()
        .to_string()
}

fn part2(r: R) -> String {
    (14..=r.t - 14)
        .filter(|hold| {
            let time = r.t - hold;
            let dis = time * hold;
            dis > r.d
        })
        .count()
        .to_string()
}

const RACE: [R; 4] = [
    R { t: 57, d: 291 },
    R { t: 72, d: 1172 },
    R { t: 69, d: 1176 },
    R { t: 92, d: 2026 },
];

fn main() {
    let numbers = part1(&RACE);
    println!("Part1: {numbers}");

    let r = R {
        t: 57726992,
        d: 291117211762026,
    };

    let numbers = part2(r);
    println!("Part2: {numbers}");
}

#[derive(Debug)]
struct R {
    t: usize,
    d: usize,
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, R};

    const SAMPLE: [R; 3] = [R { t: 7, d: 9 }, R { t: 15, d: 40 }, R { t: 30, d: 200 }];

    #[test]
    fn example_1() {
        assert_eq!(&part1(&SAMPLE), "288");
    }
    #[test]
    fn example_2() {
        assert_eq!(&part2(R { t: 71530, d: 200 }), "71503");
    }
}
