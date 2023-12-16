use std::collections::VecDeque;

use rayon::iter::{ParallelBridge, ParallelIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn get_movement(self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        }
    }

    pub fn turn_90_left(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }

    pub fn turn_90_right(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        }
    }
}

pub fn helper(maze: &[Box<[char]>], sr: usize, sc: usize, sd: Direction) -> u64 {
    let mut frontier: VecDeque<(usize, usize, Direction)> = VecDeque::from([(sr, sc, sd)]);
    let mut vis: Box<[Box<[[bool; 4]]>]> = maze
        .iter()
        .map(|r| r.iter().map(|_| [false; 4]).collect())
        .collect();

    while let Some((row, col, dir)) = frontier.pop_back() {
        if row >= maze.len() || col >= maze[0].len() || vis[row][col][dir as usize] {
            continue;
        }
        vis[row][col][dir as usize] = true;
        match maze[row][col] {
            '.' => {
                let (dr, dc) = dir.get_movement();
                frontier.push_back((row + dr as usize, col + dc as usize, dir));
            }
            '/' => {
                let new_dir = match dir {
                    Direction::South => Direction::West,
                    Direction::North => Direction::East,
                    Direction::West => Direction::South,
                    Direction::East => Direction::North,
                };
                let (dr, dc) = new_dir.get_movement();
                frontier.push_back((row + dr as usize, col + dc as usize, new_dir));
            }
            '\\' => {
                let new_dir = match dir {
                    Direction::North => Direction::West,
                    Direction::South => Direction::East,
                    Direction::East => Direction::South,
                    Direction::West => Direction::North,
                };
                let (dr, dc) = new_dir.get_movement();
                frontier.push_back((row + dr as usize, col + dc as usize, new_dir));
            }
            '|' => match dir {
                Direction::North | Direction::South => {
                    let (dr, dc) = dir.get_movement();
                    frontier.push_back((row + dr as usize, col + dc as usize, dir));
                }
                Direction::East | Direction::West => {
                    let nd1 = Direction::North;
                    let nd2 = Direction::South;

                    let (dr1, dc1) = nd1.get_movement();
                    let (dr2, dc2) = nd2.get_movement();

                    frontier.push_back((row + dr1 as usize, col + dc1 as usize, nd1));
                    frontier.push_back((row + dr2 as usize, col + dc2 as usize, nd2));
                }
            },
            '-' => match dir {
                Direction::East | Direction::West => {
                    let (dr, dc) = dir.get_movement();
                    frontier.push_back((row + dr as usize, col + dc as usize, dir));
                }
                Direction::North | Direction::South => {
                    let nd1 = Direction::East;
                    let nd2 = Direction::West;

                    let (dr1, dc1) = nd1.get_movement();
                    let (dr2, dc2) = nd2.get_movement();

                    frontier.push_back((row + dr1 as usize, col + dc1 as usize, nd1));
                    frontier.push_back((row + dr2 as usize, col + dc2 as usize, nd2));
                }
            },
            _ => unreachable!(),
        }
    }

    vis.iter()
        .flat_map(|r| r.iter())
        .filter(|&&x| x.iter().any(|&y| y))
        .count() as _
}

pub fn count_energized_tiles(maze: &str) -> u64 {
    let maze: Box<[Box<[char]>]> = maze.lines().map(|row| row.chars().collect()).collect();
    helper(&maze, 0, 0, Direction::East)
}

pub fn count_max_energized_tiles(maze: &str) -> u64 {
    let maze: Box<[Box<[char]>]> = maze.lines().map(|row| row.chars().collect()).collect();
    (0..maze.len())
        .par_bridge()
        .map(|r| {
            helper(&maze, r, 0, Direction::East).max(helper(
                &maze,
                r,
                maze[0].len() - 1,
                Direction::West,
            ))
        })
        .max()
        .unwrap()
        .max(
            (0..maze[0].len())
                .par_bridge()
                .map(|c| {
                    helper(&maze, 0, c, Direction::South).max(helper(
                        &maze,
                        maze.len() - 1,
                        c,
                        Direction::North,
                    ))
                })
                .max()
                .unwrap(),
        )
}
