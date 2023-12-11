pub fn sum_min_dists(grid: &str) -> u64 {
    let grid: Box<[Box<[char]>]> = grid.lines().map(|row| row.chars().collect()).collect();
    let mut galaxies: Vec<(i64, i64)> = vec![];
    let mut row_set: Box<[i64]> = grid.iter().map(|_| 1).collect();
    let mut col_set: Box<[i64]> = grid[0].iter().map(|_| 1).collect();
    for (ri, row) in grid.iter().enumerate() {
        for (ci, ele) in row.iter().copied().enumerate() {
            if ele == '#' {
                galaxies.push((ri as _, ci as _));
                row_set[ri] = 0;
                col_set[ci] = 0;
            }
        }
    }

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

    let mut res = 0;
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            res += (galaxies[i].0 - galaxies[j].0).abs() + (galaxies[i].1 - galaxies[j].1).abs();
        }
    }
    res as _
}

pub fn sum_min_dists_if_galaxy_much_older(grid: &str) -> u64 {
    let grid: Box<[Box<[char]>]> = grid.lines().map(|row| row.chars().collect()).collect();
    let mut galaxies: Vec<(i64, i64)> = vec![];
    let mut row_set: Box<[i64]> = grid.iter().map(|_| 999999).collect();
    let mut col_set: Box<[i64]> = grid[0].iter().map(|_| 999999).collect();
    for (ri, row) in grid.iter().enumerate() {
        for (ci, ele) in row.iter().copied().enumerate() {
            if ele == '#' {
                galaxies.push((ri as _, ci as _));
                row_set[ri] = 0;
                col_set[ci] = 0;
            }
        }
    }

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

    let mut res = 0;
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            res += (galaxies[i].0 - galaxies[j].0).abs() + (galaxies[i].1 - galaxies[j].1).abs();
        }
    }
    res as _
}
