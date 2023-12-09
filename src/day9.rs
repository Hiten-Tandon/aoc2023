pub fn oasis_value_extrapolator(input: &str) -> i64 {
    input
        .lines()
        .map(|x| {
            x.split_ascii_whitespace()
                .map(str::parse::<i64>)
                .filter_map(Result::ok)
                .collect::<Box<[_]>>()
        })
        .map(|b| {
            let mut temp = vec![b];
            while temp.last().unwrap().iter().any(|&x| x != 0) {
                temp.push(
                    temp.last()
                        .unwrap()
                        .windows(2)
                        .map(|w| w[1] - w[0])
                        .collect::<Box<[_]>>(),
                )
            }

            temp.into_iter()
                .filter_map(|x| x.last().copied())
                .fold((0, 0), |(acc, prev), ele| (acc + prev + ele, ele))
                .0
                / 2
        })
        .sum()
}
pub fn reverse_oasis_value_extrapolator(input: &str) -> i64 {
    input
        .lines()
        .map(|x| {
            x.split_ascii_whitespace()
                .map(str::parse::<i64>)
                .filter_map(Result::ok)
                .collect::<Box<[_]>>()
        })
        .map(|b| {
            let mut temp = vec![b];
            while temp.last().unwrap().iter().any(|&x| x != 0) {
                temp.push(
                    temp.last()
                        .unwrap()
                        .windows(2)
                        .map(|w| w[1] - w[0])
                        .collect::<Box<[_]>>(),
                )
            }

            let temp: Box<[i64]> = temp
                .into_iter()
                .filter_map(|x| x.first().copied())
                .collect();

            let mut res: Box<[i64]> = temp.iter().map(|_| 0).collect();

            for i in (0..res.len()).rev().skip(1) {
                res[i] = temp[i] - res[i + 1];
            }

            res[0]
        })
        .sum()
}
