use core::fmt;
use std::cmp::Ordering;

use aoc2023::{input_filename, read_input};
use winnow::{
    self,
    ascii::{digit1, line_ending},
    combinator::{eof, repeat, separated_pair, terminated},
    error::ErrMode,
    token::take_while,
    PResult, Parser,
};

#[derive(PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
struct CV(pub u8);

const CARDS: &str = "-J23456789TJQKA";

impl fmt::Display for CV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}--", CARDS.chars().nth(usize::from(self.0)).unwrap())
    }
}

impl fmt::Debug for CV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", CARDS.chars().nth(usize::from(self.0)).unwrap())
    }
}

impl CV {
    pub fn new(card: u8, j_is_joker: bool) -> Self {
        Self(match card {
            b'2' => 2,
            b'3' => 3,
            b'4' => 4,
            b'5' => 5,
            b'6' => 6,
            b'7' => 7,
            b'8' => 8,
            b'9' => 9,
            b'T' => 10,
            b'J' => {
                if j_is_joker {
                    1
                } else {
                    11
                }
            }
            b'Q' => 12,
            b'K' => 13,
            b'A' => 14,
            _ => panic!("Unknown card: {card}"),
        })
    }
}

#[derive(PartialEq, Eq)]
struct Card {
    cards: [CV; 5],
    class: Class,
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.class.cmp(&other.class) {
            core::cmp::Ordering::Equal => self.cards.cmp(&other.cards),
            ord => ord,
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.cards)
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.cards)
    }
}

impl Card {
    fn new(hand: &str, j_is_joker: bool) -> Result<Self, ErrMode<()>> {
        if hand.len() != 5 {
            return Err(ErrMode::Incomplete(winnow::error::Needed::Unknown));
        }

        let hand: [CV; 5] = hand
            .as_bytes()
            .iter()
            .map(|c| CV::new(*c, j_is_joker))
            .collect::<Vec<CV>>()
            .try_into()
            .unwrap();

        let mut cards = Vec::<(CV, u8)>::with_capacity(5);

        for &card in &hand {
            if let Some((_, cnt)) = cards.iter_mut().find(|(c, _)| *c == card) {
                *cnt += 1;
            } else {
                cards.push((card, 1));
            }
        }

        let jokers = cards
            .iter_mut()
            .find_map(|(c, cnt)| {
                if *c == CV::new(b'J', true) {
                    let c = *cnt;
                    *cnt = 0;
                    Some(c)
                } else {
                    None
                }
            })
            .unwrap_or(0);

        cards.sort_by(|a, b| {
            let o = a.1.cmp(&b.1);
            if o == Ordering::Equal {
                a.0.cmp(&b.0)
            } else {
                o
            }
        });
        cards.reverse();

        cards[0].1 += jokers;

        cards.sort_by_key(|c| c.1);
        cards.reverse();

        let class = match cards[0].1 {
            5 => Class::FiveOfKind,
            4 => Class::FourOfKind,
            1 => Class::Single,
            3 => {
                if cards[1].1 == 2 {
                    Class::FullHouse
                } else {
                    Class::ThreeOfKind
                }
            }
            2 => {
                if cards[1].1 == 2 {
                    Class::TwoPair
                } else {
                    Class::Pair
                }
            }
            e => panic!("Unknown count! {e}"),
        };

        Ok(Self { cards: hand, class })
    }
}

fn process(input: &str, j_is_joker: bool) -> PResult<Vec<(Card, u16)>> {
    let mut input = input;
    let input = &mut input;

    let dir = repeat(
        1..,
        terminated(
            separated_pair(
                take_while(5, ('2'..='9', [b'A', b'K', b'Q', b'J', b'T']))
                    .map(|hand: &str| Card::new(hand, j_is_joker).unwrap()),
                " ",
                digit1.try_map(|v: &str| v.parse::<u16>()),
            ),
            line_ending,
        ),
    )
    .parse_next(input)?;
    repeat(0.., line_ending).parse_next(input)?;
    eof.parse_next(input)?;
    Ok(dir)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum Class {
    Single = 1,
    Pair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

fn part1(input: &str) -> String {
    let mut hands = process(input, false).unwrap();

    hands.sort_by(|l, r| l.0.cmp(&r.0));

    hands
        .iter()
        .enumerate()
        .map(|(idx, (_, score))| (idx + 1) * usize::from(*score))
        .sum::<usize>()
        .to_string()
}

fn part2(input: &str) -> String {
    let mut hands = process(input, true).unwrap();

    hands.sort_by(|l, r| l.0.cmp(&r.0));

    hands
        .iter()
        .enumerate()
        .map(|(idx, (_, score))| (idx + 1) * usize::from(*score))
        .sum::<usize>()
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
    use crate::{part1, part2, process, Card, Class};

    const SAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483

";

    #[test]
    fn process_data() {
        let j_is_joker: bool = false;

        let g = process(SAMPLE, j_is_joker);

        assert_eq!(
            g,
            Ok(vec![
                (Card::new("32T3K", j_is_joker).unwrap(), 765),
                (Card::new("T55J5", j_is_joker).unwrap(), 684),
                (Card::new("KK677", j_is_joker).unwrap(), 28),
                (Card::new("KTJJT", j_is_joker).unwrap(), 220),
                (Card::new("QQQJA", j_is_joker).unwrap(), 483)
            ])
        );

        assert_eq!(
            Card::new("AAAAA", j_is_joker).unwrap().class,
            Class::FiveOfKind
        );
        assert_eq!(
            Card::new("AA8AA", j_is_joker).unwrap().class,
            Class::FourOfKind
        );
        assert_eq!(
            Card::new("23332", j_is_joker).unwrap().class,
            Class::FullHouse
        );
        assert_eq!(
            Card::new("TTT98", j_is_joker).unwrap().class,
            Class::ThreeOfKind
        );
        assert_eq!(
            Card::new("23432", j_is_joker).unwrap().class,
            Class::TwoPair
        );
        assert_eq!(Card::new("32T3K", j_is_joker).unwrap().class, Class::Pair);
        assert_eq!(Card::new("A23A4", j_is_joker).unwrap().class, Class::Pair);
        assert_eq!(Card::new("23456", j_is_joker).unwrap().class, Class::Single);

        let mut hands = g.unwrap();

        hands.sort_by(|l, r| l.0.cmp(&r.0));
        hands.reverse();

        assert_eq!(
            hands,
            vec![
                (Card::new("QQQJA", j_is_joker).unwrap(), 483),
                (Card::new("T55J5", j_is_joker).unwrap(), 684),
                (Card::new("KK677", j_is_joker).unwrap(), 28),
                (Card::new("KTJJT", j_is_joker).unwrap(), 220),
                (Card::new("32T3K", j_is_joker).unwrap(), 765),
            ]
        );
    }

    #[test]
    fn process_data_p2() {
        let j_is_joker = true;
        let mut hands = process(SAMPLE, j_is_joker).unwrap();

        hands.sort_by(|l, r| l.0.cmp(&r.0));

        assert_eq!(
            hands,
            vec![
                (Card::new("32T3K", j_is_joker).unwrap(), 765),
                (Card::new("KK677", j_is_joker).unwrap(), 28),
                (Card::new("T55J5", j_is_joker).unwrap(), 684),
                (Card::new("QQQJA", j_is_joker).unwrap(), 483),
                (Card::new("KTJJT", j_is_joker).unwrap(), 220),
            ]
        );
    }

    #[test]
    fn example_1() {
        assert_eq!(&part1(SAMPLE), "6440");
    }
    #[test]
    fn example_2() {
        assert_eq!(&part2(SAMPLE), "5905");
    }
}
