use std::collections::{HashSet, VecDeque};

pub fn step_counter_for_elf(mat: &str) -> u64 {
    let (mut s_x, mut s_y) = (0, 0);
    let mat: Box<[Box<[char]>]> = mat
        .lines()
        .enumerate()
        .map(|(i, r)| {
            r.char_indices()
                .map(|(j, e)| {
                    if e == 'S' {
                        (s_x, s_y) = (i, j);
                    }
                    e
                })
                .collect()
        })
        .collect();
    let mut visited: Box<[Box<[bool]>]> = mat
        .iter()
        .map(|r| r.iter().map(|_| false).collect())
        .collect();

    let mut frontier: VecDeque<(u8, usize, usize)> = VecDeque::from([(64, s_x, s_y)]);
    let mut count = 0;

    while let Some((steps, row, col)) = frontier.pop_front() {
        if row >= mat.len() || col >= mat.len() || mat[row][col] == '#' || visited[row][col] {
            continue;
        }

        visited[row][col] = true;
        if steps % 2 == 0 {
            count += 1;
        }

        if steps != 0 {
            frontier.push_back((steps - 1, row, col - 1));
            frontier.push_back((steps - 1, row, col + 1));
            frontier.push_back((steps - 1, row - 1, col));
            frontier.push_back((steps - 1, row + 1, col));
        }
    }

    count
}

pub fn step_counter_for_chad_elf(mat: &str) -> u64 {
    let (mut s_x, mut s_y) = (0, 0);
    let mat: Box<[Box<[char]>]> = mat
        .lines()
        .enumerate()
        .map(|(i, r)| {
            r.char_indices()
                .map(|(j, e)| {
                    if e == 'S' {
                        (s_x, s_y) = (i, j);
                    }
                    e
                })
                .collect()
        })
        .collect();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut frontier: VecDeque<(u64, usize, usize)> = VecDeque::from([(26_501_365, s_x, s_y)]);
    let mut count = 0;

    while let Some((steps, mut row, mut col)) = frontier.pop_back() {
        if mat[row % mat.len()][col % mat[0].len()] == '#' || visited.contains(&(row, col)) {
            continue;
        }

        visited.insert((row, col));
        if steps % 2 == 0 {
            count += 1;
            if count % 1_000_000 == 0 {
                println!("{count}");
            }
        }

        if row == 0 {
            row = mat.len();
        }

        if col == 0 {
            col = mat[0].len();
        }

        if steps != 0 {
            frontier.push_back((steps - 1, row, col - 1));
            frontier.push_back((steps - 1, row, col + 1));
            frontier.push_back((steps - 1, row - 1, col));
            frontier.push_back((steps - 1, row + 1, col));
        }
    }

    count
}
