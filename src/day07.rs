use std::str::FromStr;

use itertools::Itertools;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(usize)]
enum Card {
    SJ,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Deck {
    HighCard(Box<[Card]>),
    OnePair(Box<[Card]>),
    TwoPair(Box<[Card]>),
    ThreeOfAKind(Box<[Card]>),
    FullHouse(Box<[Card]>),
    FourOfAKind(Box<[Card]>),
    FiveOfAKind(Box<[Card]>),
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        use Card::*;
        match value {
            'A' => A,
            'K' => K,
            'Q' => Q,
            'J' => J,
            'T' => T,
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            'j' => SJ,
            _ => unreachable!(),
        }
    }
}

impl From<Card> for char {
    fn from(value: Card) -> Self {
        match value {
            Card::Two => '2',
            Card::Three => '3',
            Card::Four => '4',
            Card::Five => '5',
            Card::Six => '6',
            Card::Seven => '7',
            Card::Eight => '8',
            Card::Nine => '9',
            Card::T => 'T',
            Card::J => 'J',
            Card::Q => 'Q',
            Card::K => 'K',
            Card::A => 'A',
            Card::SJ => 'j',
        }
    }
}

impl FromStr for Deck {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: Box<[Card]> = s.chars().map(Card::from).collect();
        let mut counts: [u8; 14] = [0; 14];
        let mut kinds: u8 = 0;
        let mut sj_count = 0;
        cards.iter().for_each(|&card| {
            if card != Card::SJ && counts[card as usize] == 0 {
                kinds += 1;
            }

            if card == Card::SJ {
                sj_count += 1;
            } else {
                counts[card as usize] += 1
            }
        });

        if sj_count == 5 {
            kinds = 1;
        }

        *counts.iter_mut().max().unwrap() += sj_count;

        Ok(match kinds {
            1 => Deck::FiveOfAKind(cards),
            5 => Deck::HighCard(cards),
            2 => {
                let mut c = 0;
                for count in counts.iter().copied() {
                    if count == 1 || count == 2 {
                        c = count;
                    }
                }

                if c == 1 {
                    Deck::FourOfAKind(cards)
                } else {
                    Deck::FullHouse(cards)
                }
            }
            3 => {
                let (c1, c2, c3) = counts
                    .iter()
                    .copied()
                    .filter(|&c| c != 0)
                    .collect_tuple()
                    .unwrap_or_default();

                match (c1, c2, c3) {
                    (2, 2, 1) | (2, 1, 2) | (1, 2, 2) => Deck::TwoPair(cards),
                    (3, 1, 1) | (1, 3, 1) | (1, 1, 3) => Deck::ThreeOfAKind(cards),
                    _ => unreachable!(),
                }
            }
            4 => Deck::OnePair(cards),
            _ => unreachable!(),
        })
    }
}

pub fn get_total_winnings(set_bid_pairs: &str) -> u64 {
    let mut biddings: Box<[_]> = set_bid_pairs
        .lines()
        .map(|line| line.split_ascii_whitespace().collect_tuple().unwrap())
        .map(|(hand, bid)| (hand.parse::<Deck>().unwrap(), bid.parse::<u64>().unwrap()))
        .collect();

    biddings.sort_unstable();

    biddings
        .iter()
        .enumerate()
        .map(|(i, &(_, bid))| bid * (i + 1) as u64)
        .sum()
}

pub fn get_total_winnings_with_joker(set_bid_pairs: &str) -> u64 {
    let mut biddings: Box<[_]> = set_bid_pairs
        .replace('J', "j")
        .lines()
        .map(|line| line.split_ascii_whitespace().collect_tuple().unwrap())
        .map(|(hand, bid)| (hand.parse::<Deck>().unwrap(), bid.parse::<u64>().unwrap()))
        .collect();

    biddings.sort_unstable();
    // println!("{biddings:#?}");
    biddings
        .iter()
        .enumerate()
        .map(|(i, &(_, bid))| bid * (i + 1) as u64)
        .sum()
}
