use itertools::Itertools;
type Pair<T> = (T, T);
type Triplet<T> = (T, T, T);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Cube {
    x_range: (u64, u64),
    y_range: (u64, u64),
    z_range: (u64, u64),
    supporter: Vec<usize>,
    supported_by: Vec<usize>,
}

impl From<Pair<Triplet<u64>>> for Cube {
    fn from(((x1, y1, z1), (x2, y2, z2)): Pair<Triplet<u64>>) -> Self {
        Self {
            x_range: (x1, x2),
            y_range: (y1, y2),
            z_range: (z1, z2),
            supporter: vec![],
            supported_by: vec![],
        }
    }
}

impl Cube {
    fn shallow_copy(&self) -> Self {
        Self {
            supporter: vec![],
            supported_by: vec![],
            x_range: self.x_range,
            y_range: self.y_range,
            z_range: self.z_range,
        }
    }
}
pub fn count_destructable_blocks(falling_snapshot: &str) -> u64 {
    let mut falling_snapshot: Vec<Cube> = falling_snapshot
        .lines()
        .filter_map(|line| {
            line.split('~')
                .filter_map(|cords| {
                    cords
                        .split(',')
                        .map(str::parse::<u64>)
                        .filter_map(Result::ok)
                        .collect_tuple::<Triplet<_>>()
                })
                .collect_tuple::<Pair<_>>()
        })
        .map(Cube::from)
        .collect();

    falling_snapshot.iter().for_each(|x| println!("{x:?}"));

    let mut frontier: Vec<(usize, usize, Cube)> = vec![(
        0,
        falling_snapshot.len() + 1,
        Cube::from(((0, 0, 0), (1_000_000, 1_000_000, 0))),
    )];
    while let Some((curr, prev_id, prev_surface)) = frontier.pop() {
        if (prev_surface.x_range.0..=prev_surface.x_range.1)
            .contains(&falling_snapshot[curr].x_range.0)
            || (prev_surface.x_range.0..=prev_surface.x_range.1)
                .contains(&falling_snapshot[curr].x_range.1)
                && (prev_surface.y_range.0..=prev_surface.y_range.1)
                    .contains(&falling_snapshot[curr].y_range.0)
            || (prev_surface.y_range.0..=prev_surface.y_range.1)
                .contains(&falling_snapshot[curr].y_range.1)
        {
            let diff = falling_snapshot[curr].z_range.1 - falling_snapshot[curr].z_range.0;
            falling_snapshot[curr].z_range.0 = prev_surface.z_range.1 + 1;
            falling_snapshot[curr].z_range.1 = falling_snapshot[curr].z_range.0 + diff;
            falling_snapshot[curr].supporter.push(prev_id);
            frontier.push((curr + 1, curr, falling_snapshot[curr].shallow_copy()));
        }
    }
    0
}
