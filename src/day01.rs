pub fn sum_purified_data(impure_data: &str) -> u64 {
    impure_data
        .lines()
        .map(|line| line.chars().filter(char::is_ascii_digit))
        .map(|x| (x.clone().next().unwrap() as u8 - b'0') * 10 + x.last().unwrap() as u8 - b'0')
        .map(u64::from)
        .sum()
}

pub fn sum_correct_purified_data(impure_data: &str) -> u64 {
    sum_purified_data(
        impure_data
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e")
            .as_ref(),
    )
}
