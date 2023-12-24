use std::collections::BinaryHeap;

use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Direction {
    North = b'^',
    West = b'<',
    South = b'v',
    East = b'>',
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Tile {
    Blocked = b'#',
    Slope(Direction),
    AnyDirection = b'.',
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Tile::AnyDirection,
            '^' => Tile::Slope(Direction::North),
            '>' => Tile::Slope(Direction::East),
            '<' => Tile::Slope(Direction::West),
            'v' => Tile::Slope(Direction::South),
            '#' => Tile::Blocked,
            _ => Err(())?,
        })
    }
}

pub fn get_longest_hike_route_length(map: &str) -> u64 {
    let map: Box<[Box<[Tile]>]> = map
        .lines()
        .map(|r| {
            r.chars()
                .map(|x| Tile::try_from(x))
                .filter_map(Result::ok)
                .collect()
        })
        .collect();

    let mut frontier: Vec<(Vec<(usize, usize)>, u64, usize, usize)> = vec![(vec![], 0, 1, 1)];

    let mut res = 0;
    while let Some((mut visited, step_count, row, col)) = frontier.pop() {
        if row >= map.len() || col >= map[row].len() || visited.contains(&(row, col)) {
            continue;
        }

        if row == map.len() - 1 {
            res = res.max(step_count);
        }

        visited.push((row, col));

        match map[row][col] {
            Tile::Blocked => (),
            Tile::Slope(s) => match s {
                Direction::North => frontier.push((visited.clone(), step_count + 1, row - 1, col)),
                Direction::West => frontier.push((visited.clone(), step_count + 1, row, col - 1)),
                Direction::South => frontier.push((visited.clone(), step_count + 1, row + 1, col)),
                Direction::East => frontier.push((visited.clone(), step_count + 1, row, col + 1)),
            },
            Tile::AnyDirection => frontier.extend([
                (visited.clone(), step_count + 1, row - 1, col),
                (visited.clone(), step_count + 1, row, col - 1),
                (visited.clone(), step_count + 1, row + 1, col),
                (visited.clone(), step_count + 1, row, col + 1),
            ]),
        }
    }
    res
}

fn helper(mat: &[Box<[Tile]>], mut path: Vec<(usize, usize)>, row: usize, col: usize) -> u64 {
    if row >= mat.len()
        || col >= mat[0].len()
        || mat[row][col] == Tile::Blocked
        || path.contains(&(row, col))
    {
        return 0;
    }

    path.push((row, col));
    if row == mat.len() - 1 {
        println!("{}", path.len());
        return path.len() as _;
    }

    vec![
        (mat, path.clone(), row, col - 1),
        (mat, path.clone(), row, col + 1),
        (mat, path.clone(), row - 1, col),
        (mat, path.clone(), row + 1, col),
    ]
    .into_iter()
    .map(|(x, y, r, c)| helper(x, y, r, c))
    .filter(|&x| x != 0)
    .max()
    .unwrap_or(0)
}

pub fn get_longest_hike_route_length_without_slopes(map: &str) -> u64 {
    let map: Box<[Box<[Tile]>]> = map
        .lines()
        .map(|r| {
            r.chars()
                .map(|x| Tile::try_from(x))
                .filter_map(Result::ok)
                .collect()
        })
        .collect();

    helper(&map, vec![], 1, 1)
}
