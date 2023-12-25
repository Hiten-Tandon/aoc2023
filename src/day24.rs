use itertools::Itertools;

pub fn count_intersections_of_hailstone_trajectories(input_conditions: &str) -> u64 {
    input_conditions
        .lines()
        .filter_map(|l| {
            l.split('@')
                .filter_map(|x| {
                    x.split(',')
                        .map(str::trim)
                        .map(str::parse::<f64>)
                        .filter_map(Result::ok)
                        .take(2)
                        .collect_tuple()
                })
                .collect_tuple()
        })
        .combinations(2)
        .map(|v| {
            let ((x1, y1), (dx1, dy1)) = v[0];
            let ((x2, y2), (dx2, dy2)) = v[1];

            let m1 = dy1 / dx1;
            let m2 = dy2 / dx2;

            if m1 == m2 {
                return 0;
            }
            let c1 = y1 - (m1 * x1);
            let c2 = y2 - (m2 * x2);

            let collision_x = -(c1 - c2) / (m1 - m2);
            let collision_y = m1 * collision_x + c1;

            let t1 = y1 / dy1;
            let tc1 = collision_y / dy1;

            let t2 = y2 / dy2;
            let tc2 = collision_y / dy2;
            if (200000000000000.0 <= collision_x && collision_x <= 400000000000000.0)
                && (200000000000000.0 <= collision_y && collision_y <= 400000000000000.0)
                && tc1 > t1
                && tc2 > t2
            {
                1
            } else {
                0
            }
        })
        .sum::<u64>()
}
pub fn get_stone_position(input_conditions: &str) -> u64 {
    let v = input_conditions
        .lines()
        .filter_map(|l| {
            l.split('@')
                .filter_map(|x| {
                    x.split(',')
                        .map(str::trim)
                        .map(str::parse::<f64>)
                        .filter_map(Result::ok)
                        .collect_tuple::<(_, _, _)>()
                })
                .collect_tuple::<(_, _)>()
        })
        .take(3)
        .collect_vec();

    print!("(xr - {}) / (yr - {}) = ", v[0].0 .0, v[0].0 .1);
    print!("(xr - {}) / (yr - {}) = ", v[1].0 .0, v[1].0 .1);
    println!("(xr - {}) / (yr - {})", v[2].0 .0, v[2].0 .1);
    print!("(zr - {}) / (yr - {}) = ", v[0].0 .2, v[0].0 .1);
    print!("(zr - {}) / (yr - {}) = ", v[1].0 .2, v[1].0 .1);
    println!("(zr - {}) / (yr - {})", v[2].0 .2, v[2].0 .1);
    0
}
