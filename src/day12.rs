use itertools::Itertools;
use std::collections::HashMap;

fn helper(rep: &str, counts: &[usize], cache: &mut HashMap<(Box<str>, Box<[usize]>), u64>) -> u64 {
    if rep.is_empty() {
        u64::from(counts.is_empty())
    } else if counts.is_empty() {
        u64::from(!rep.contains('#'))
    } else if let Some(&res) = cache.get(&(rep.into(), counts.as_ref().into())) {
        res
    } else {
        let mut res = 0;

        if rep.starts_with(|x| x == '.' || x == '?') {
            res += helper(&rep[1..], counts, cache);
        }

        if rep.starts_with(|x| x == '#' || x == '?') {
            if counts[0] <= rep.len()
                && !rep[..counts[0]].contains('.')
                && (counts[0] == rep.len() || rep.chars().nth(counts[0]) != Some('#'))
            {
                res += helper(
                    if counts[0] == rep.len() {
                        ""
                    } else {
                        &rep[(counts[0] + 1)..]
                    },
                    &counts[1..],
                    cache,
                );
            }
        }

        cache.insert((rep.into(), counts.as_ref().into()), res);
        res
    }
}

pub fn possible_configs_of_broken_stuff(data: &str) -> u64 {
    let mut cache = HashMap::new();
    data.lines()
        .map(|line| {
            cache.clear();
            let (broken_part, ok_part) = line.split_ascii_whitespace().collect_tuple().unwrap();
            let ok_part: Box<[usize]> = ok_part
                .split(',')
                .map(str::trim)
                .map(str::parse::<usize>)
                .filter_map(Result::ok)
                .collect();
            helper(broken_part, &ok_part, &mut cache)
        })
        .sum()
}

pub fn possible_configs_of_copied_broken_stuff(data: &str) -> u64 {
    let mut cache = HashMap::new();
    data.lines()
        .map(|line| {
            let (broken_part, ok_part) = line.split_ascii_whitespace().collect_tuple().unwrap();
            let broken_part = [broken_part; 5].join("?");
            let ok_part: Box<[usize]> = [ok_part; 5]
                .join(",")
                .split(',')
                .map(str::trim)
                .map(str::parse::<usize>)
                .filter_map(Result::ok)
                .collect();

            helper(&broken_part, &ok_part, &mut cache)
        })
        .sum()
}
