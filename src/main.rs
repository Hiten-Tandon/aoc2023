use std::{
    fs::File,
    io::{stdin, Read},
    time::Instant,
};

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day7_v2;

use crate::{
    day1::{sum_correct_purified_data, sum_purified_data},
    day2::{get_possible_games, get_power_games},
    day3::{sum_gear_ratios, sum_parts},
    day4::{count_total_cards, get_winning_points},
    day5::{get_lowest_seed_location, get_lowest_seed_location_from_ranges},
    day6::{count_ways_to_win, count_ways_to_win_if_one_race},
    day7_v2::{get_total_winnings, get_total_winnings_with_joker},
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
        "5 1" => println!("{}", get_lowest_seed_location(&f_cont)),
        "5 2" => println!("{}", get_lowest_seed_location_from_ranges(&f_cont)),
        "6 1" => println!("{}", count_ways_to_win(&f_cont)),
        "6 2" => println!("{}", count_ways_to_win_if_one_race(&f_cont)),
        "7 1" => println!("{}", get_total_winnings(&f_cont)),
        "7 2" => println!("{}", get_total_winnings_with_joker(&f_cont)),
        _ => unreachable!(),
    }

    println!("{:#?}", t.elapsed());
    Ok(())
}
