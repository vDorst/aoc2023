use aoc2023::{input_filename, read_input};

#[derive(Debug)]
pub enum C {
    Blue(u8),
    Red(u8),
    Green(u8),
}

impl C {
    pub fn from(data: &str) -> Self {
        let (count, colour) = data.trim().split_once(' ').expect("Need a space!");

        let count: u8 = count.parse().expect("Valid number");

        match colour {
            "blue" => C::Blue(count),
            "green" => C::Green(count),
            "red" => C::Red(count),
            _ => panic!("Invalid colour {colour}"),
        }
    }
}

#[derive(Debug)]
struct Game {
    num: u32,
    cubes: Vec<Vec<C>>,
}

fn part1(data: &str) -> String {
    let games = data
        .lines()
        .filter(|s| !s.is_empty())
        .filter_map(|line| {
            let (game, cubes) = line.split_once(':').expect("Find a :");
            let game_num = game
                .split_once(' ')
                .unwrap()
                .1
                .parse()
                .expect("game number");
            let cubes = cubes
                .split(';')
                .map(|hand| hand.split(',').map(C::from).collect::<Vec<C>>())
                .collect::<Vec<Vec<C>>>();

            // filter invalid games
            if cubes.iter().flatten().any(|cude| match cude {
                C::Blue(num) => *num > 14,
                C::Red(num) => *num > 12,
                C::Green(num) => *num > 13,
            }) {
                None
            } else {
                Some(Game {
                    num: game_num,
                    cubes,
                })
            }
        })
        .collect::<Vec<Game>>();

    games.iter().fold(0, |acc, game| acc + game.num).to_string()
}

fn part2(data: &str) -> String {
    data.lines()
        .filter(|s| !s.is_empty())
        .map(|line| {
            let (_game, cubes) = line.split_once(':').expect("Find a :");

            let cubes = cubes
                .split(';')
                .map(|hand| hand.split(',').map(C::from).collect::<Vec<C>>())
                .collect::<Vec<Vec<C>>>();

            let mut blue = 0;
            let mut red = 0;
            let mut green = 0;

            // find the highest cubes per colour
            cubes.iter().flatten().for_each(|cude| match cude {
                C::Blue(num) => blue = blue.max(*num),
                C::Red(num) => red = red.max(*num),
                C::Green(num) => green = green.max(*num),
            });

            let ans = u32::from(blue) * u32::from(red) * u32::from(green);
            ans
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

    const SAMPLE: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    #[test]
    fn example_1() {
        assert_eq!(&part1(SAMPLE), "8");
    }
    #[test]
    fn example_2() {
        assert_eq!(&part2(SAMPLE), "2286");
    }
}
