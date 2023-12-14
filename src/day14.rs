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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RockType {
    Rounded,
    Cubic,
}

pub fn calculate_load_on_north_beams_after_cycles(position_map: &str) -> u64 {
    todo!()
}
