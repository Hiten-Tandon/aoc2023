use onig::*;

pub fn sum_purified_data(impure_data: &str) -> u64 {
    impure_data
        .lines()
        .map(|line| line.chars().filter(|x| x.is_digit(10)))
        .map(|x| (x.clone().next().unwrap() as u8 - b'0') * 10 + x.last().unwrap() as u8 - b'0')
        .map(u64::from)
        .sum()
}

const NUMS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn sum_correct_purified_data(impure_data: &str) -> u64 {
    let regex: Regex =
        Regex::new((String::from("(?=(") + &NUMS.join("|") + "|\\d))").as_str()).unwrap();

    let f = |x: &str| -> u64 {
        x.parse::<u64>().unwrap_or_else(|_| {
            NUMS.iter()
                .position(|&y| x == y)
                .unwrap_or(usize::max_value()) as u64
                + 1_u64
        })
    };

    impure_data
        .lines()
        .map(|line| {
            let v: Vec<_> = regex.captures_iter(line).collect();
            //println!("{v:?}");
            let first = v.first().unwrap();
            let last = v.last().unwrap();

            (
                f(first.iter().collect::<Vec<_>>()[1].unwrap()),
                f(last.iter().collect::<Vec<_>>()[1].unwrap()),
            )
        })
        .map(|(a, b)| a * 10 + b)
        .sum()
}
