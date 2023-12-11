use itertools::Itertools;

pub fn sum_min_dists(grid: &str) -> u64 {
    let mut galaxies: Vec<(i64, i64)> = vec![];
    let mut row_set: Box<[i64]> = grid.lines().map(|_| 1).collect();
    let mut col_set: Box<[i64]> = grid.lines().next().unwrap().chars().map(|_| 1).collect();

    grid.lines().enumerate().for_each(|(ri, row)| {
        row.char_indices()
            .filter(|&(_, e)| e == '#')
            .for_each(|(ci, _)| {
                galaxies.push((ri as _, ci as _));
                row_set[ri] = 0;
                col_set[ci] = 0;
            })
    });

    for i in 1..row_set.len() {
        row_set[i] += row_set[i - 1];
    }
    for i in 1..col_set.len() {
        col_set[i] += col_set[i - 1];
    }

    galaxies.iter_mut().for_each(|x| {
        x.0 += row_set[x.0 as usize];
        x.1 += col_set[x.1 as usize];
    });

    galaxies
        .into_iter()
        .tuple_combinations()
        .map(|((x1, y1), (x2, y2))| (x1 - x2).abs() + (y1 - y2).abs())
        .sum::<i64>() as _
}

pub fn sum_min_dists_if_galaxy_much_older(grid: &str) -> u64 {
    let mut galaxies: Vec<(i64, i64)> = vec![];

    let mut row_set: Box<[i64]> = grid.lines().map(|_| 999999).collect();
    let mut col_set: Box<[i64]> = grid
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|_| 999999)
        .collect();

    grid.lines().enumerate().for_each(|(ri, row)| {
        row.char_indices()
            .filter(|&(_, e)| e == '#')
            .for_each(|(ci, _)| {
                galaxies.push((ri as _, ci as _));
                row_set[ri] = 0;
                col_set[ci] = 0;
            })
    });

    for i in 1..row_set.len() {
        row_set[i] += row_set[i - 1];
    }
    for i in 1..col_set.len() {
        col_set[i] += col_set[i - 1];
    }

    galaxies.iter_mut().for_each(|x| {
        x.0 += row_set[x.0 as usize];
        x.1 += col_set[x.1 as usize];
    });

    galaxies
        .into_iter()
        .tuple_combinations()
        .map(|((x1, y1), (x2, y2))| (x1 - x2).abs() + (y1 - y2).abs())
        .sum::<i64>() as _
}
