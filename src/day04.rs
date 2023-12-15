use std::ops::BitOr;

use itertools::Itertools;

pub fn get_winning_points(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            line.split(':')
                .nth(1)
                .unwrap()
                .split('|')
                .map(|x| {
                    x.split_ascii_whitespace()
                        .map(str::parse::<u8>)
                        .filter_map(Result::ok)
                        .map(|x| 1 << x)
                        .fold(0, u128::bitor)
                })
                .collect_tuple()
                .unwrap_or_default()
        })
        .map(|(a, b)| a & b)
        .filter(|&x| x != 0)
        .map(|x| 1 << (x.count_ones() - 1))
        .sum::<usize>() as _
}

pub fn count_total_cards(input: &str) -> u64 {
    let mut dp: Box<[u64]> = vec![1; input.lines().count()].into();
    input
        .lines()
        .map(|line| {
            line.split(':')
                .nth(1)
                .unwrap()
                .split('|')
                .map(|x| {
                    x.split_ascii_whitespace()
                        .map(str::parse::<u8>)
                        .filter_map(Result::ok)
                        .map(|x| 1 << x)
                        .fold(0, u128::bitor)
                })
                .collect_tuple()
                .unwrap_or_default()
        })
        .map(|(a, b)| (a & b).count_ones() as _)
        .enumerate()
        .for_each(|(pos, count)| {
            let mul = dp[pos];
            dp.iter_mut()
                .skip(pos + 1)
                .take(count)
                .for_each(|e| *e += mul)
        });
    dp.iter().sum()
}
