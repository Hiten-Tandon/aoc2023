use itertools::Itertools;

pub fn count_intersections_of_hailstone_trajectories(input_conditions: &str) -> u64 {
    input_conditions
        .lines()
        .filter_map(|l| {
            l.split('@')
                .filter_map(|x| {
                    x.split(',')
                        .map(str::trim)
                        .map(str::parse::<i64>)
                        .filter_map(Result::ok)
                        .collect_tuple()
                })
                .collect_tuple()
        })
        .for_each(|((x, y, z), (dx, dy, dz))| println!("{:?}, {:?}", (x, y, z), (dx, dy, dz)));
    0
}
