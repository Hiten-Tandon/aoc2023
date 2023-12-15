use std::{
    fs::File,
    io::{stdin, Read},
    time::Instant,
};

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day12_v2;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day7_v2;
pub mod day8;
pub mod day9;

use crate::{
    day1::{sum_correct_purified_data, sum_purified_data},
    day10::{count_tiles_enclosed_by_loop, find_loop_length},
    day11::{sum_min_dists, sum_min_dists_if_galaxy_much_older},
    day12::{possible_configs_of_broken_stuff, possible_configs_of_copied_broken_stuff},
    day13::{evaluate_point_of_incidence, evaluate_point_of_incidence_after_removing_smudge},
    day14::{
        calculate_load_on_north_beams_after_cycles, calculate_load_on_north_beams_after_tilting,
    },
    day15::{get_power_of_lenses_after_creating_hashmap, sum_holiday_hash_for_testing_purposes},
    day2::{get_possible_games, get_power_games},
    day3::{sum_gear_ratios, sum_parts},
    day4::{count_total_cards, get_winning_points},
    day5::{get_lowest_seed_location, get_lowest_seed_location_from_ranges},
    day6::{count_ways_to_win, count_ways_to_win_if_one_race},
    day7_v2::{get_total_winnings, get_total_winnings_with_joker},
    day8::{get_ghost_path_length, get_path_length},
    day9::{oasis_value_extrapolator, reverse_oasis_value_extrapolator},
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
        "8 1" => println!("{}", get_path_length(&f_cont)),
        "8 2" => println!("{}", get_ghost_path_length(&f_cont)),
        "9 1" => println!("{}", oasis_value_extrapolator(&f_cont)),
        "9 2" => println!("{}", reverse_oasis_value_extrapolator(&f_cont)),
        "10 1" => println!("{}", find_loop_length(&f_cont)),
        "10 2" => println!("{}", count_tiles_enclosed_by_loop(&f_cont)),
        "11 1" => println!("{}", sum_min_dists(&f_cont)),
        "11 2" => println!("{}", sum_min_dists_if_galaxy_much_older(&f_cont)),
        "12 1" => println!("{}", possible_configs_of_broken_stuff(&f_cont)),
        "12 2" => println!("{}", possible_configs_of_copied_broken_stuff(&f_cont)),

        "13 1" => println!("{}", evaluate_point_of_incidence(&f_cont)),
        "13 2" => println!(
            "{}",
            evaluate_point_of_incidence_after_removing_smudge(&f_cont)
        ),
        "14 1" => println!("{}", calculate_load_on_north_beams_after_tilting(&f_cont)),
        "14 2" => println!("{}", calculate_load_on_north_beams_after_cycles(&f_cont)),
        "15 1" => println!("{}", sum_holiday_hash_for_testing_purposes(&f_cont)),
        "15 2" => println!("{}", get_power_of_lenses_after_creating_hashmap(&f_cont)),
        _ => unreachable!(),
    }

    println!("{:#?}", t.elapsed());
    Ok(())
}
