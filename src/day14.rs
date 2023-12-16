use std::collections::HashMap;

use itertools::Itertools;

pub fn calculate_load_on_north_beams_after_tilting(position_map: &str) -> u64 {
    let mut position_map: Box<[Box<[_]>]> = position_map
        .lines()
        .map(|line| line.bytes().collect())
        .collect();

    let mut sweeper: Box<[_]> = position_map[0].iter().map(|_| 0).collect();

    position_map.iter_mut().rev().for_each(|row| {
        row.iter_mut()
            .zip(sweeper.iter_mut())
            .for_each(|(ele, s_ele)| match *ele {
                b'#' => {
                    *ele = *s_ele;
                    *s_ele = 0
                }
                b'O' => {
                    *s_ele += 1;
                    *ele = 0
                }
                _ => *ele = 0,
            })
    });

    sweeper
        .iter()
        .copied()
        .map(u64::from)
        .map(|x| (0..x).map(|p| position_map.len() as u64 - p).sum::<u64>())
        .sum::<u64>()
        + position_map
            .iter()
            .rev()
            .enumerate()
            .map(|(ri, row)| {
                row.iter()
                    .copied()
                    .map(u64::from)
                    .map(|x| (0..x).map(|p| ri as u64 - p).sum::<u64>())
                    .sum::<u64>()
            })
            .sum::<u64>()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RockType {
    Rounded,
    Cubic,
}

fn cycle(position_map: &[(usize, usize, RockType)]) -> Box<[(usize, usize, RockType)]> {
    let mut position_map: Box<[_]> = position_map.into();

    position_map.sort_by_key(|&(r, c, _)| (c, r));
    let mut cr = 0;
    for (r, _, ele) in position_map.iter_mut() {
        match *ele {
            RockType::Cubic => cr = *r,
            RockType::Rounded => {
                cr += 1;
                *r = cr;
            }
        }
    }

    position_map.sort_by_key(|&(r, c, _)| (r, c));
    let mut cc = 0;
    for (_, c, ele) in position_map.iter_mut() {
        match *ele {
            RockType::Cubic => cc = *c,
            RockType::Rounded => {
                cc += 1;
                *c = cc;
            }
        }
    }

    position_map.sort_by_key(|&(r, c, _)| (c, !r));
    let mut cr = 0;
    for (r, _, ele) in position_map.iter_mut() {
        match *ele {
            RockType::Cubic => cr = *r,
            RockType::Rounded => {
                cr -= 1;
                *r = cr;
            }
        }
    }

    position_map.sort_by_key(|&(r, c, _)| (r, !c));
    let mut cc = 0;
    for (_, c, ele) in position_map.iter_mut() {
        match *ele {
            RockType::Cubic => cc = *c,
            RockType::Rounded => {
                cc -= 1;
                *c = cc;
            }
        }
    }
    position_map
}
pub fn calculate_load_on_north_beams_after_cycles(position_map: &str) -> u64 {
    let col_count = position_map.lines().next().unwrap().len();
    let row_count = position_map.lines().count();

    let position_map = format!(
        "{}\n{position_map}{}",
        "#".repeat(col_count),
        "#".repeat(col_count)
    );
    let mut position_map: Box<[(usize, usize, RockType)]> = position_map
        .lines()
        .enumerate()
        .flat_map(move |(ri, row)| {
            format!("#{row}#")
                .char_indices()
                .filter_map(move |(ci, x)| {
                    Some(match x {
                        'O' => (ri, ci, RockType::Rounded),
                        '#' => (ri, ci, RockType::Cubic),
                        _ => None?,
                    })
                })
                .collect_vec()
        })
        .collect();

    let mut graph: HashMap<
        Box<[(usize, usize, RockType)]>,
        (usize, Box<[(usize, usize, RockType)]>),
    > = HashMap::new();

    let mut pos = 0;
    while graph.get(&position_map).is_none() {
        let next_state = cycle(&position_map);
        graph.insert(position_map, (pos, next_state.clone()));
        pos += 1;
        position_map = next_state;
    }

    let const_diff = graph.get(&position_map).unwrap().0;
    let cycle_len = pos - const_diff;

    let f_count = if cycle_len == 0 {
        0
    } else {
        (1_000_000_000 - const_diff) % cycle_len
    };

    for _ in 0..f_count {
        position_map = graph.get(&position_map).unwrap().1.clone();
    }

    position_map
        .iter()
        .copied()
        .filter(|&(_, _, t)| t == RockType::Rounded)
        .map(|(r, _, _)| row_count - r + 1)
        .sum::<usize>() as _
}
