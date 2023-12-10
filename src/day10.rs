use itertools::Itertools;
use std::{collections::VecDeque, convert::identity as id, str::FromStr};

type BoxSlice<T> = Box<[T]>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Pipe {
    VerticalPipe = b'|',
    HorizontalPipe = b'-',
    TRBentPipe = b'J',
    TLBentPipe = b'L',
    BRBentPipe = b'7',
    BLBentPipe = b'F',
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    GroundTile,
    StartTile,
    PipeTile(Pipe),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MapParseErr {
    JaggeredGrid,
    InvalidTile,
    NoStartTile,
    NonDeterministicStartTile,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GroundMap {
    grid: BoxSlice<BoxSlice<Tile>>,
    start_pos: (usize, usize),
}

impl Pipe {
    pub fn get_attaching_positions(
        self,
        (x, y): (usize, usize),
    ) -> ((usize, usize), (usize, usize)) {
        match self {
            Pipe::VerticalPipe => ((x, y - 1), (x, y + 1)),
            Pipe::HorizontalPipe => ((x - 1, y), (x + 1, y)),
            Pipe::TRBentPipe => ((x, y - 1), (x + 1, y)),
            Pipe::TLBentPipe => ((x - 1, y), (x, y - 1)),
            Pipe::BRBentPipe => ((x, y + 1), (x + 1, y)),
            Pipe::BLBentPipe => ((x - 1, y), (x, y + 1)),
        }
    }
}

impl TryFrom<char> for Pipe {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '|' => Pipe::VerticalPipe,
            '-' => Pipe::HorizontalPipe,
            'J' => Pipe::TLBentPipe,
            'L' => Pipe::TRBentPipe,
            '7' => Pipe::BLBentPipe,
            'F' => Pipe::BRBentPipe,
            _ => Err("Not a valid pipe")?,
        })
    }
}

impl TryFrom<char> for Tile {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'S' => Tile::StartTile,
            '.' => Tile::GroundTile,
            x => Tile::PipeTile(x.try_into()?),
        })
    }
}

impl FromStr for GroundMap {
    type Err = MapParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid: BoxSlice<BoxSlice<Tile>> = s
            .lines()
            .map(|x| x.chars().map(Tile::try_from).collect::<Result<_, _>>())
            .collect::<Result<_, _>>()
            .map_err(|_| MapParseErr::InvalidTile)?;

        if grid.iter().any(|r| r.len() != grid[0].len()) {
            return Err(MapParseErr::JaggeredGrid);
        }

        let mut start_pos: Option<(usize, usize)> = None;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == Tile::StartTile {
                    start_pos = Some((i, j));
                }
            }
        }

        let start_pos = start_pos.ok_or(MapParseErr::NoStartTile)?;
        let mut neighbors = [
            (start_pos.0, start_pos.1 - 1),
            (start_pos.0, start_pos.1 + 1),
            (start_pos.0 - 1, start_pos.1),
            (start_pos.0 + 1, start_pos.1),
        ]
        .iter()
        .copied()
        .filter_map(|(x, y)| {
            grid.get(x)
                .map(|r| r.get(y).copied().map(|e| (x, y, e)))
                .flatten()
        })
        .filter_map(|(x, y, e)| {
            if let Tile::PipeTile(p) = e {
                Some((x, y, p))
            } else {
                None
            }
        })
        .filter_map(|(r, c, p)| {
            let (p1, p2) = p.get_attaching_positions((c, r));
            if p1 == (start_pos.1, start_pos.0) || p2 == (start_pos.1, start_pos.0) {
                Some((r, c))
            } else {
                None
            }
        })
        .collect_vec();

        if neighbors.len() > 2 {
            return Err(MapParseErr::NonDeterministicStartTile);
        }

        if neighbors.len() == 1 {
            grid[start_pos.0][start_pos.1] = Tile::PipeTile(match neighbors.pop().unwrap() {
                (x, _) if x == start_pos.0 + 1 || x == start_pos.0 - 1 => Pipe::HorizontalPipe,
                _ => Pipe::VerticalPipe,
            });
        }

        if neighbors.len() == 2 {
            grid[start_pos.0][start_pos.1] =
                Tile::PipeTile(match (neighbors.pop().unwrap(), neighbors.pop().unwrap()) {
                    ((r1, _), (r2, _)) if (r1 + r2) == start_pos.0 * 2 => Pipe::VerticalPipe,
                    ((_, c1), (_, c2)) if (c1 + c2) == start_pos.1 * 2 => Pipe::HorizontalPipe,
                    ((r1, c1), (r2, c2))
                        if [(r1, c2), (r2, c1)].contains(&(start_pos.0 + 1, start_pos.1 + 1)) =>
                    {
                        Pipe::BRBentPipe
                    }
                    ((r1, c1), (r2, c2))
                        if [(r1, c2), (r2, c1)].contains(&(start_pos.0 - 1, start_pos.1 + 1)) =>
                    {
                        Pipe::TRBentPipe
                    }
                    ((r1, c1), (r2, c2))
                        if [(r1, c2), (r2, c1)].contains(&(start_pos.0 + 1, start_pos.1 - 1)) =>
                    {
                        Pipe::BLBentPipe
                    }
                    ((r1, c1), (r2, c2))
                        if [(r1, c2), (r2, c1)].contains(&(start_pos.0 - 1, start_pos.1 - 1)) =>
                    {
                        Pipe::TLBentPipe
                    }
                    _ => Pipe::VerticalPipe,
                })
        }
        Ok(Self { grid, start_pos })
    }
}

impl GroundMap {
    pub fn get_distances(&self) -> BoxSlice<BoxSlice<Option<u64>>> {
        let mut dists: BoxSlice<BoxSlice<_>> = self
            .grid
            .iter()
            .map(|r| r.iter().map(|_| None).collect())
            .collect();
        let mut frontier: VecDeque<(usize, usize, u64)> =
            VecDeque::from([(self.start_pos.0, self.start_pos.1, 0)]);

        while let Some((r, c, dist)) = frontier.pop_front() {
            if r >= self.grid.len() || c >= self.grid[0].len() || dists[r][c].is_some() {
                continue;
            }
            let Tile::PipeTile(pipe) = self.grid[r][c] else {
                continue;
            };
            dists[r][c] = Some(dist);
            let ((c1, r1), (c2, r2)) = pipe.get_attaching_positions((c, r));
            frontier.push_back((r1, c1, dist + 1));
            frontier.push_back((r2, c2, dist + 1));
        }

        dists
    }
    pub fn count_tiles_enclosed_by_loop(&self) -> u64 {
        self.get_distances()
            .iter()
            .zip(self.grid.iter())
            .map(|(dist_rows, row)| {
                let mut temp: Vec<Tile> = vec![];
                dist_rows
                    .iter()
                    .copied()
                    .zip(row.iter().copied())
                    .map(|(a, b)| if a.is_some() { b } else { Tile::GroundTile })
                    .filter(|&t| t != Tile::PipeTile(Pipe::HorizontalPipe))
                    .for_each(|x| match (temp.pop(), x) {
                        (
                            Some(Tile::PipeTile(Pipe::TRBentPipe)),
                            Tile::PipeTile(Pipe::BLBentPipe),
                        )
                        | (
                            Some(Tile::PipeTile(Pipe::BRBentPipe)),
                            Tile::PipeTile(Pipe::TLBentPipe),
                        ) => {
                            temp.push(Tile::try_from('|').unwrap());
                        }
                        (y, x) => {
                            y.map(|t| temp.push(t));
                            temp.push(x);
                        }
                    });
                let res = temp
                    .into_iter()
                    .scan(0, |acc, t| {
                        Some(match t {
                            Tile::GroundTile | Tile::StartTile => *acc % 2,
                            Tile::PipeTile(Pipe::VerticalPipe) => {
                                *acc += 1;
                                0
                            }
                            _ => 0,
                        })
                    })
                    .sum::<u64>();
                res
            })
            .sum::<u64>()
    }
}

pub fn find_loop_length(ground_map: &str) -> u64 {
    ground_map
        .parse::<GroundMap>()
        .unwrap()
        .get_distances()
        .iter()
        .map(|x| x.iter())
        .flatten()
        .copied()
        .filter_map(id)
        .max()
        .unwrap()
}

pub fn count_tiles_enclosed_by_loop(ground_map: &str) -> u64 {
    ground_map
        .parse::<GroundMap>()
        .unwrap()
        .count_tiles_enclosed_by_loop()
}
