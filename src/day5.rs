use std::ops::Range;

use itertools::Itertools;

pub fn get_lowest_seed_location(data: &str) -> u64 {
    let mut lines = data.lines();
    let mut chunks: Vec<Box<[Box<str>]>> = vec![];

    while lines.clone().count() != 0 {
        chunks.push(
            lines
                .by_ref()
                .take_while(|x| !x.is_empty())
                .map(|x| Box::from(x))
                .collect(),
        )
    }

    chunks.reverse();

    let mut curr_vals: Box<[u64]> = chunks.pop().unwrap()[0]
        .split(':')
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(str::parse::<u64>)
        .filter_map(Result::ok)
        .collect();

    while let Some(map) = chunks.pop() {
        let map: Box<[(Range<u64>, u64, u64)]> = map
            .into_iter()
            .skip(1)
            .map(|x| {
                x.split_ascii_whitespace()
                    .map(str::parse::<u64>)
                    .filter_map(Result::ok)
                    .collect_tuple()
                    .unwrap_or_default()
            })
            .map(|(v, k, count)| (k..(k + count), k, v))
            .collect();

        for i in 0..curr_vals.len() {
            if let Some((_, k, v)) = map
                .iter()
                .cloned()
                .find(|(k_range, _, _)| k_range.contains(&curr_vals[i]))
            {
                curr_vals[i] = curr_vals[i] - k + v;
            }
        }
    }
    curr_vals.into_iter().min().copied().unwrap()
}

pub fn get_lowest_seed_location_from_ranges(data: &str) -> u64 {
    let mut lines = data.lines();
    let mut chunks: Vec<Vec<Box<str>>> = vec![];

    while lines.clone().count() != 0 {
        chunks.push(
            lines
                .by_ref()
                .take_while(|x| !x.is_empty())
                .map(|x| Box::from(x))
                .collect(),
        )
    }

    chunks.reverse();

    let mut curr_vals: Vec<(u64, u64)> = chunks.pop().unwrap()[0]
        .split(':')
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(str::parse::<u64>)
        .filter_map(Result::ok)
        .chunks(2)
        .into_iter()
        .filter_map(|x| x.collect_tuple())
        .map(|(a, b)| (a, a + b))
        .collect();

    while let Some(map) = chunks.pop() {
        let map: Box<[(u64, u64, u64)]> = map
            .into_iter()
            .skip(1)
            .map(|x| {
                x.split_ascii_whitespace()
                    .map(str::parse::<u64>)
                    .filter_map(Result::ok)
                    .collect_tuple()
                    .unwrap_or_default()
            })
            .map(|(v, k, count)| (k, v, count))
            .collect();

        let mut temp = vec![];
        while let Some((s, e)) = curr_vals.pop() {
            if let Some((k, v, _)) = map.iter().copied().find(|&(k, _, r)| s >= k && e <= k + r) {
                temp.push((s - k + v, e - k + v));
            } else if let Some((k, v, r)) =
                map.iter().copied().find(|&(k, _, r)| s <= k && e >= k + r)
            {
                if s != k {
                    curr_vals.push((s, k));
                }
                temp.push((v, v + r));
                if e != k + r {
                    curr_vals.push((k + r, e));
                }
            } else if let Some((k, v, r)) = map
                .iter()
                .copied()
                .find(|&(k, _, r)| (k..(k + r)).contains(&s))
            {
                temp.push((s - k + v, v + r));
                curr_vals.push((k + r, e));
            } else if let Some((k, v, _)) = map
                .iter()
                .copied()
                .find(|&(k, _, r)| ((k + 1)..=(k + r)).contains(&e))
            {
                temp.push((v, e - k + v));
                curr_vals.push((s, k));
            } else {
                temp.push((s, e));
            }
        }
        curr_vals = temp;
    }
    curr_vals.into_iter().map(|(a, _)| a).min().unwrap()
}
