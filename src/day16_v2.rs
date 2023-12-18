use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mirror {
    Vertical,
    Horizontal,
    LeftTilted,
    RightTilted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl TryFrom<char> for Mirror {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            '/' => Self::RightTilted,
            '\\' => Self::LeftTilted,
            _ => Err(())?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Node {
    mirror: Mirror,
    row_major_pos: (usize, usize),
    col_major_pos: (usize, usize),
}

impl Node {
    pub fn new(
        mirror: Mirror,
        row_major_pos: (usize, usize),
        col_major_pos: (usize, usize),
    ) -> Self {
        Self {
            mirror,
            row_major_pos,
            col_major_pos,
        }
    }
}

impl ToString for Mirror {
    fn to_string(&self) -> String {
        match self {
            Mirror::Vertical => "|",
            Mirror::Horizontal => "-",
            Mirror::LeftTilted => "\\",
            Mirror::RightTilted => "/",
        }
        .into()
    }
}

impl ToString for Node {
    fn to_string(&self) -> String {
        self.mirror.to_string()
    }
}

impl Node {
    #[inline(always)]
    pub fn get_actual_pos(self) -> (usize, usize) {
        (self.row_major_pos.0, self.col_major_pos.0)
    }
}

pub fn count_energized_tiles(maze: &str) -> u64 {
    let mut col_mat = vec![vec![]; maze.lines().next().unwrap().len()];
    let row_mat = maze
        .lines()
        .enumerate()
        .map(|(ri, row)| {
            let mut rc = 0;
            row.char_indices()
                .filter_map(|(ci, c)| Some((ci, Mirror::try_from(c).ok()?)))
                .map(|(ci, m)| {
                    let res = Node::new(m, (ri, rc), (ci, col_mat[ci].len()));
                    col_mat[ci].push(res);
                    rc += 1;
                    res
                })
                .collect_vec()
        })
        .collect_vec();

    let get_next_in_dir: [&dyn Fn(Node) -> Option<Node>; 4] = [
        &|node: Node| {
            col_mat[node.col_major_pos.0]
                .get(node.col_major_pos.1 - 1)
                .copied()
        },
        &|node: Node| {
            col_mat[node.col_major_pos.0]
                .get(node.col_major_pos.1 + 1)
                .copied()
        },
        &|node: Node| {
            row_mat[node.row_major_pos.0]
                .get(node.row_major_pos.1 + 1)
                .copied()
        },
        &|node: Node| {
            row_mat[node.row_major_pos.0]
                .get(node.row_major_pos.1 - 1)
                .copied()
        },
    ];
    let mut visiting_history: Vec<((usize, usize), (usize, usize))> = vec![(
        (0, 0),
        (row_mat[0][0].row_major_pos.0, row_mat[0][0].col_major_pos.0),
    )];
    let mut visited: Vec<Vec<u8>> = row_mat
        .iter()
        .map(|r| r.iter().map(|_| 0).collect_vec())
        .collect_vec();
    let mut frontier: Vec<(Node, Direction)> = vec![(row_mat[0][0], Direction::East)];

    while let Some((node, dir)) = frontier.pop() {
        println!("{}, {dir:?}", node.to_string());
        if visited[node.row_major_pos.0][node.row_major_pos.1] >> dir as u8 & 1 == 1 {
            continue;
        }
        visited[node.row_major_pos.0][node.row_major_pos.1] |= 1 << dir as u8;

        match (node.mirror, dir) {
            (Mirror::Vertical, Direction::North) => {
                let Some(next) = get_next_in_dir[Direction::North as usize](node) else {
                    visiting_history.push((node.get_actual_pos(), (0, node.col_major_pos.0)));
                    continue;
                };
                visiting_history.push((node.get_actual_pos(), next.get_actual_pos()));
                frontier.push((next, Direction::North));
            }
            (Mirror::Vertical, Direction::South) => {
                let Some(next) = get_next_in_dir[Direction::South as usize](node) else {
                    visiting_history.push((
                        node.get_actual_pos(),
                        (row_mat.len() - 1, node.col_major_pos.0),
                    ));
                    continue;
                };
                visiting_history.push((node.get_actual_pos(), next.get_actual_pos()));
                frontier.push((next, Direction::South));
            }
            (Mirror::Vertical, Direction::East) | (Mirror::Vertical, Direction::West) => {
                if let Some(next) = get_next_in_dir[Direction::South as usize](node) {
                    visiting_history.push((node.get_actual_pos(), next.get_actual_pos()));
                    frontier.push((next, Direction::South));
                } else {
                    visiting_history.push((
                        node.get_actual_pos(),
                        (row_mat.len() - 1, node.col_major_pos.0),
                    ));
                }

                if let Some(next) = get_next_in_dir[Direction::North as usize](node) {
                    visiting_history.push((node.get_actual_pos(), next.get_actual_pos()));
                    frontier.push((next, Direction::North));
                } else {
                    visiting_history.push((node.get_actual_pos(), (0, node.col_major_pos.0)));
                }
            }
            (Mirror::Horizontal, Direction::North) | (Mirror::Horizontal, Direction::South) => {
                if let Some(next) = get_next_in_dir[Direction::East as usize](node) {
                    visiting_history.push((node.get_actual_pos(), next.get_actual_pos()));
                    frontier.push((next, Direction::East));
                } else {
                    visiting_history.push((
                        node.get_actual_pos(),
                        (node.row_major_pos.0, col_mat.len() - 1),
                    ));
                }
                if let Some(next) = get_next_in_dir[Direction::West as usize](node) {
                    visiting_history.push((node.get_actual_pos(), next.get_actual_pos()));
                    frontier.push((next, Direction::West));
                } else {
                    visiting_history.push((node.get_actual_pos(), (node.row_major_pos.0, 0)));
                }
            }
            (Mirror::Horizontal, Direction::East) => {
                let Some(next) = get_next_in_dir[Direction::East as usize](node) else {
                    visiting_history.push((
                        node.get_actual_pos(),
                        (node.row_major_pos.0, col_mat.len() - 1),
                    ));
                    continue;
                };
                visiting_history.push((node.get_actual_pos(), next.get_actual_pos()));
                frontier.push((next, Direction::East));
            }
            (Mirror::Horizontal, Direction::West) => {
                let Some(next) = get_next_in_dir[Direction::West as usize](node) else {
                    visiting_history.push((node.get_actual_pos(), (node.row_major_pos.0, 0)));
                    continue;
                };
                visiting_history.push((node.get_actual_pos(), next.get_actual_pos()));
                frontier.push((next, Direction::West));
            }
            (Mirror::RightTilted, Direction::South) | (Mirror::LeftTilted, Direction::North) => {
                let Some(next) = get_next_in_dir[Direction::West as usize](node) else {
                    visiting_history.push((node.get_actual_pos(), (node.row_major_pos.0, 0)));
                    continue;
                };
                visiting_history.push((node.get_actual_pos(), next.get_actual_pos()));
                frontier.push((next, Direction::West));
            }
            (Mirror::RightTilted, Direction::North) | (Mirror::LeftTilted, Direction::South) => {
                let Some(next) = get_next_in_dir[Direction::East as usize](node) else {
                    visiting_history.push((
                        node.get_actual_pos(),
                        (node.row_major_pos.0, col_mat.len() - 1),
                    ));
                    continue;
                };
                visiting_history.push((node.get_actual_pos(), next.get_actual_pos()));
                frontier.push((next, Direction::East));
            }
            (Mirror::RightTilted, Direction::East) | (Mirror::LeftTilted, Direction::West) => {
                let Some(next) = get_next_in_dir[Direction::North as usize](node) else {
                    visiting_history.push((node.get_actual_pos(), (0, node.col_major_pos.0)));
                    continue;
                };
                visiting_history.push((node.get_actual_pos(), next.get_actual_pos()));
                frontier.push((next, Direction::North));
            }
            (Mirror::RightTilted, Direction::West) | (Mirror::LeftTilted, Direction::East) => {
                let Some(next) = get_next_in_dir[Direction::South as usize](node) else {
                    visiting_history.push((
                        node.get_actual_pos(),
                        (row_mat.len() - 1, node.col_major_pos.0),
                    ));
                    continue;
                };
                visiting_history.push((node.get_actual_pos(), next.get_actual_pos()));
                frontier.push((next, Direction::South));
            }
        }
    }
    // println!("Row:");
    // row_mat.iter().for_each(|r| {
    //     r.iter().for_each(|(_, n)| print!("{} ", n.to_string()));
    //     println!()
    // });
    // println!("Col:");
    // col_mat.iter().for_each(|r| {
    //     r.iter().for_each(|(_, n)| print!("{} ", n.to_string()));
    //     println!()
    // });

    let mut visited: Box<[Box<[bool]>]> = maze
        .lines()
        .map(|x| x.chars().map(|_| false).collect())
        .collect();

    for ((r1, c1), (r2, c2)) in visiting_history {
        for r in r1..=r2 {
            for c in c1..=c2 {
                visited[r][c] = true;
            }
        }
    }

    for i in 0..visited.len() {
        for j in 0..visited[i].len() {
            print!("{}", if visited[i][j] { "#" } else { "." });
        }
        println!()
    }
    visited
        .iter()
        .flat_map(|x| x.iter())
        .filter(|&&x| x)
        .count() as _
}

pub fn count_max_energized_tiles(_maze: &str) -> u64 {
    todo!()
}
