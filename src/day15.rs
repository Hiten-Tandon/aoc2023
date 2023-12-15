use std::array;

use itertools::Itertools;

fn holiday_hash_algorithm(x: &str) -> u64 {
    x.bytes().filter(|&y| y != b'\n').fold(0, |mut a, c| {
        a += c as u64;
        a *= 17;
        a % 256
    })
}

pub fn sum_holiday_hash_for_testing_purposes(input: &str) -> u64 {
    input.split(',').map(holiday_hash_algorithm).sum()
}

pub fn get_power_of_lenses_after_creating_hashmap(input: &str) -> u64 {
    let mut hash_map: [Vec<(&str, u64)>; 256] = array::from_fn(|_| vec![]);
    input.split(',').filter(|x| !x.is_empty()).for_each(|op| {
        if op.contains('=') {
            let (key, val) = op.split("=").collect_tuple().unwrap();
            let hash = holiday_hash_algorithm(key) as usize;
            let val = val.trim().parse::<u64>().unwrap();
            if let Some(p) = hash_map[hash].iter_mut().find(|(k, _)| *k == key) {
                p.1 = val;
            } else {
                hash_map[hash].push((key.into(), val));
            }
        } else {
            let key = op.split('-').next().unwrap();
            let hash = holiday_hash_algorithm(key) as usize;
            if let Some(p) = hash_map[hash].iter().position(|&(k, _)| k == key) {
                hash_map[hash].remove(p);
            }
        }
    });

    hash_map
        .iter()
        .enumerate()
        .map(|(box_id, r#box)| {
            r#box
                .iter()
                .enumerate()
                .map(|(pos, ele)| ((box_id + 1) * (pos + 1)) as u64 * ele.1)
                .sum::<u64>()
        })
        .sum()
}
