use itertools::Itertools;

const CARDS: &'static str = "j23456789TJQKA";
fn encode(c: char) -> u32 {
    CARDS.find(c).unwrap() as _
}

pub fn get_total_winnings(set_bid_pairs: &str) -> u64 {
    set_bid_pairs
        .lines()
        .map(|pair| pair.split_ascii_whitespace().collect_tuple().unwrap())
        .map(|(deck, val)| {
            let mut counts: [u8; 14] = [0; 14];
            let deck = deck.chars().map(encode).fold(0, |acc, code| {
                counts[code as usize] += 1;
                (acc << 4) | code
            });
            counts.sort_by_key(|x| !x);
            let p = match counts {
                [5, ..] => 6,
                [4, 1, ..] => 5,
                [3, 2, ..] => 4,
                [3, 1, ..] => 3,
                [2, 2, ..] => 2,
                [2, 1, ..] => 1,
                _ => 0,
            };

            ((p << 20) | deck, val.parse::<u64>().unwrap())
        })
        .sorted_unstable()
        .enumerate()
        .map(|(i, (_, val))| val * (i + 1) as u64)
        .sum()
}

pub fn decode(mut x: u32) -> Box<str> {
    let mut res = String::new();

    for _ in 0..5 {
        res.push(CARDS.chars().nth((x & 15) as _).unwrap());
        x >>= 4;
    }

    res.chars().rev().collect::<String>().into()
}
pub fn get_total_winnings_with_joker(set_bid_pairs: &str) -> u64 {
    set_bid_pairs
        .lines()
        .map(|pair| pair.split_ascii_whitespace().collect_tuple().unwrap())
        .map(|(deck, val)| {
            let mut counts: [u8; 14] = [0; 14];
            let mut j_count = 0;
            let deck = deck
                .chars()
                .map(|x| if x == 'J' { 'j' } else { x })
                .map(encode)
                .fold(0, |acc, code| {
                    if code != 0 {
                        counts[code as usize] += 1;
                    } else {
                        j_count += 1;
                    }
                    (acc << 4) | code
                });
            if j_count == 5 {
                counts[0] = j_count;
            } else {
                *counts.iter_mut().max().unwrap() += j_count;
                counts.sort_by_key(|x| !x);
            }
            let p = match counts {
                [5, ..] => 6,
                [4, ..] => 5,
                [3, 2, ..] => 4,
                [3, 1, ..] => 3,
                [2, 2, ..] => 2,
                [2, 1, ..] => 1,
                _ => 0,
            };
            ((p << 20) | deck, val.parse::<u64>().unwrap())
        })
        .sorted()
        .enumerate()
        .map(|(i, (_, val))| val * (i + 1) as u64)
        .sum()
}
