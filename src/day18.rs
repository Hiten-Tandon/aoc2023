use itertools::Itertools;

pub fn colorize_perimeter(instructions: &str) -> i64 {
    let mut boundary: i64 = 0;
    let points = instructions
        .lines()
        .filter_map(|line| line.split_ascii_whitespace().take(2).collect_tuple())
        .filter_map(|(dir, dist)| {
            let dist = dist.parse::<i64>().unwrap();
            boundary += dist;
            Some(match dir {
                "U" => (0, -dist),
                "D" => (0, dist),
                "L" => (-dist, 0),
                "R" => (dist, 0),
                _ => None?,
            })
        })
        .scan((0, 0), |acc, (dx, dy)| {
            acc.0 += dx;
            acc.1 += dy;
            Some(*acc)
        })
        .collect_vec();
    let a1: i64 = points
        .iter()
        .copied()
        .cycle()
        .tuple_windows()
        .take(points.len())
        .map(|((_, y1), (x2, _), (_, y3))| x2 * (y3 - y1))
        .sum::<i64>()
        .abs()
        / 2;

    println!("{a1}, {boundary}");
    a1 - boundary / 2 + 1 + boundary
}
pub fn colorize_larger_perimeter(instructions: &str) -> i64 {
    let mut boundary: i64 = 0;
    let points = instructions
        .lines()
        .filter_map(|line| line.split_ascii_whitespace().skip(2).next())
        .map(|instruction| {
            (
                &instruction[instruction.len() - 2..instruction.len() - 1],
                &instruction[2..instruction.len() - 2],
            )
        })
        .filter_map(|(dir, dist)| {
            let dist = i64::from_str_radix(dist, 16).ok()?;
            boundary += dist;
            Some(match dir {
                "3" => (0, -dist),
                "1" => (0, dist),
                "2" => (-dist, 0),
                "0" => (dist, 0),
                _ => None?,
            })
        })
        .scan((0, 0), |acc, (dx, dy)| {
            acc.0 += dx;
            acc.1 += dy;
            Some(*acc)
        })
        .collect_vec();
    let a1: i64 = points
        .iter()
        .copied()
        .cycle()
        .tuple_windows()
        .take(points.len())
        .map(|((_, y1), (x2, _), (_, y3))| (x2 * (y3 - y1)))
        .sum::<i64>()
        .abs()
        / 2;

    println!("{a1}, {boundary}");
    a1 - boundary / 2 + 1 + boundary
}
