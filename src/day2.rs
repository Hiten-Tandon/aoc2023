use std::str::FromStr;

use itertools::Itertools;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Game {
    red_count: u64,
    green_count: u64,
    blue_count: u64,
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut res: Self = Self {
            red_count: 0,
            green_count: 0,
            blue_count: 0,
        };
        s.split(',')
            .map(|x| {
                x.split_whitespace()
                    .collect_tuple::<(&str, &str)>()
                    .unwrap()
            })
            .for_each(|(count, ty)| match ty.trim() {
                "red" => res.red_count += count.parse::<u64>().unwrap(),
                "blue" => res.blue_count += count.parse::<u64>().unwrap(),
                "green" => res.green_count += count.parse::<u64>().unwrap(),
                _ => {
                    unreachable!()
                }
            });
        Ok(res)
    }
}
pub fn get_possible_games(game_list: &str) -> u64 {
    game_list
        .lines()
        .map(|game| game.split(':').collect_tuple().unwrap())
        .map(|(game_code, game_moves)| {
            (
                game_code
                    .split_whitespace()
                    .nth(1)
                    .unwrap()
                    .parse::<u64>()
                    .unwrap(),
                game_moves.split(';').map(|x| x.parse::<Game>().unwrap()),
            )
        })
        .filter_map(|(code, mut move_iter)| {
            if move_iter.all(|x| x.red_count <= 12 && x.blue_count <= 14 && x.green_count <= 13) {
                Some(code)
            } else {
                None
            }
        })
        .sum()
}

pub fn get_power_games(game_list: &str) -> u64 {
    game_list
        .lines()
        .map(|game| game.split(':').collect_tuple().unwrap())
        .map(|(game_code, game_moves)| {
            (
                game_code
                    .split_whitespace()
                    .nth(1)
                    .unwrap()
                    .parse::<u64>()
                    .unwrap(),
                game_moves.split(';').map(|x| x.parse::<Game>().unwrap()),
            )
        })
        .map(|(_e, move_iter)| {
            move_iter.fold(Game::default(), |acc, g| Game {
                red_count: acc.red_count.max(g.red_count),
                green_count: acc.green_count.max(g.green_count),
                blue_count: acc.blue_count.max(g.blue_count),
            })
        })
        .map(|g| g.red_count * g.blue_count * g.green_count)
        .sum()
}
