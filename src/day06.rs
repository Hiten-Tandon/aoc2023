use itertools::Itertools;
use std::ops::Mul;

pub fn count_ways_to_win(dist_time: &str) -> u64 {
    let (times, dists): (Box<[f64]>, Box<[f64]>) = dist_time
        .lines()
        .map(|x| {
            x.split_ascii_whitespace()
                .map(str::parse::<f64>)
                .filter_map(Result::ok)
                .collect()
        })
        .collect_tuple()
        .unwrap();

    times
        .iter()
        .copied()
        .zip(dists.iter().copied())
        .map(|(tt, d)| {
            (
                (-tt + (tt.powi(2) - (4.0 * d)).sqrt()) / -2.0,
                (-tt - (tt.powi(2) - (4.0 * d)).sqrt()) / -2.0,
            )
        })
        .map(|(a, b)| {
            b.floor() as u64
                - if a == a.floor() {
                    a.floor() as u64 + 1
                } else {
                    a.floor() as u64
                }
        })
        .fold(1, u64::mul)
}

pub fn count_ways_to_win_if_one_race(dist_time: &str) -> u64 {
    let (tt, d): (f64, f64) = dist_time
        .lines()
        .map(|x| {
            x.split(':')
                .nth(1)
                .unwrap()
                .replace(' ', "")
                .parse::<f64>()
                .unwrap()
        })
        .collect_tuple()
        .unwrap();

    let (a, b) = (
        (-tt + (tt.powi(2) - (4.0 * d)).sqrt()) / -2.0,
        (-tt - (tt.powi(2) - (4.0 * d)).sqrt()) / -2.0,
    );

    b.floor() as u64
        - if a == a.floor() {
            a.floor() as u64 + 1
        } else {
            a.floor() as u64
        }
}
