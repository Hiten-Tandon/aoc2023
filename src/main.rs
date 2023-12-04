use std::{
    fs::File,
    io::{stdin, Read},
    time::Instant,
};
pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

use crate::{
    day1::{sum_correct_purified_data, sum_purified_data},
    day2::{get_possible_games, get_power_games},
    day3::{sum_gear_ratios, sum_parts},
    day4::{count_total_cards, get_winning_points},
};

fn main() -> Result<(), ()> {
    let mut input = String::new();
    println!("Enter day and version, seperated by space: ");
    let _ = stdin().read_line(&mut input);
    let Ok(mut f) = File::open(
        String::from("inputs/") + input.split_whitespace().next().unwrap().trim() + ".txt",
    ) else {
        return Err(());
    };
    let mut f_cont = String::new();
    let _ = f.read_to_string(&mut f_cont);

    let t: Instant = Instant::now();
    match input.trim() {
        "1 1" => println!("{}", sum_purified_data(&f_cont)),
        "1 2" => println!("{}", sum_correct_purified_data(&f_cont)),
        "2 1" => println!("{}", get_possible_games(&f_cont)),
        "2 2" => println!("{}", get_power_games(&f_cont)),
        "3 1" => println!("{}", sum_parts(&f_cont)),
        "3 2" => println!("{}", sum_gear_ratios(&f_cont)),
        "4 1" => println!("{}", get_winning_points(&f_cont)),
        "4 2" => println!("{}", count_total_cards(&f_cont)),
        _ => unreachable!(),
    }

    println!("{:#?}", t.elapsed());
    Ok(())
}
