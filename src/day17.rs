use std::{cmp::Reverse, collections::BinaryHeap};

pub fn minimize_crucible_temperature_loss(map: &str) -> u64 {
    let grid: Box<[Box<[u64]>]> = map
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0').map(u64::from).collect())
        .collect();

    let mut frontier = BinaryHeap::with_capacity(grid.len().pow(2));
    frontier.push(Reverse((0, 0, 0, 0, 0, 0)));
    let mut seen: Box<[Box<[_]>]> = grid
        .iter()
        .map(|r| r.iter().map(|_| [[[false; 4]; 3]; 3]).collect())
        .collect();

    while let Some(Reverse((hl, r, c, dr, dc, n))) = frontier.pop() {
        if r == grid.len() - 1 && c == grid[0].len() - 1 {
            return hl;
        }

        if seen[r][c][dr + 1][dc + 1][n] {
            continue;
        }

        seen[r][c][dr + 1][dc + 1][n] = true;

        if n < 3 && (dr, dc) != (0, 0) {
            let (nr, nc) = (r + dr, c + dc);
            if nr < grid.len() && nc < grid[0].len() {
                frontier.push(Reverse((hl + grid[nr][nc], nr, nc, dr, dc, n + 1)));
            }
        }

        for (ndr, ndc) in [
            (0, 1),
            (1, 0),
            (0, usize::max_value()),
            (usize::max_value(), 0),
        ] {
            if (ndr, ndc) != (dr, dc) && (ndr, ndc) != (!dr + 1, !dc + 1) {
                let (nr, nc) = (r + ndr, c + ndc);
                if nr < grid.len() && nc < grid[0].len() {
                    frontier.push(Reverse((hl + grid[nr][nc], nr, nc, ndr, ndc, 1)));
                }
            }
        }
    }
    0
}
pub fn minimize_stable_crucible_temperature_loss(map: &str) -> u64 {
    let grid: Box<[Box<[u64]>]> = map
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0').map(u64::from).collect())
        .collect();

    let mut frontier = BinaryHeap::with_capacity(grid.len().pow(2));
    frontier.push(Reverse((0, 0, 0, 0, 0, 0)));
    let mut seen: Box<[Box<[_]>]> = grid
        .iter()
        .map(|r| r.iter().map(|_| [[[false; 11]; 3]; 3]).collect())
        .collect();

    while let Some(Reverse((hl, r, c, dr, dc, n))) = frontier.pop() {
        if r == grid.len() - 1 && c == grid[0].len() - 1 && n > 4 {
            return hl;
        }

        if seen[r][c][dr + 1][dc + 1][n] {
            continue;
        }

        seen[r][c][dr + 1][dc + 1][n] = true;

        if n < 10 && (dr, dc) != (0, 0) {
            let (nr, nc) = (r + dr, c + dc);
            if nr < grid.len() && nc < grid[0].len() {
                frontier.push(Reverse((hl + grid[nr][nc], nr, nc, dr, dc, n + 1)));
            }
        }

        if n >= 4 || (dr, dc) == (0, 0) {
            for (ndr, ndc) in [
                (0, 1),
                (1, 0),
                (0, usize::max_value()),
                (usize::max_value(), 0),
            ] {
                if (ndr, ndc) != (dr, dc) && (ndr, ndc) != (!dr + 1, !dc + 1) {
                    let (nr, nc) = (r + ndr, c + ndc);
                    if nr < grid.len() && nc < grid[0].len() {
                        frontier.push(Reverse((hl + grid[nr][nc], nr, nc, ndr, ndc, 1)));
                    }
                }
            }
        }
    }
    0
}
