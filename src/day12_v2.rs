use itertools::Itertools;

pub fn possible_configs_of_broken_stuff(data: &str) -> u64 {
    data.lines()
        .filter_map(|line| line.split_ascii_whitespace().collect_tuple())
        .map(|(a, b)| {
            (
                a.split('.')
                    .filter(|x| !x.is_empty())
                    .enumerate()
                    .map(|(i, x)| {
                        let p_count = x.chars().filter(|&y| y == '#').count();
                        (i, x, p_count, x.len() - p_count)
                    })
                    .collect::<Box<[_]>>(),
                b.split(',')
                    .map(|x| x.trim())
                    .map(str::parse::<u64>)
                    .filter_map(Result::ok)
                    .collect::<Box<[_]>>(),
            )
        })
        .map(|(rep, counts)| {
            let mut last_min = 0;
            let mut temp = counts
                .iter()
                .copied()
                .map(|count| {
                    let mut curr_min = usize::max_value();
                    let res = rep
                        .iter()
                        .copied()
                        .filter(|(_, _, p, q)| p + q >= count as usize)
                        .filter(move |&(pos, _, _, _)| {
                            let res = pos >= last_min;
                            if res {
                                curr_min = pos.min(curr_min)
                            }
                            res
                        })
                        .collect::<Box<[_]>>();
                    res
                })
                .collect::<Box<[_]>>();

            println!("{temp:?}");
            0
        })
        .sum()
}

pub fn possible_configs_of_copied_broken_stuff(data: &str) -> u64 {
    todo!()
}
