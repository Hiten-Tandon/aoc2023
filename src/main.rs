use std::{
    fs::File,
    io::{stdin, Read},
    time::Instant,
};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day07_v2;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day16_v2;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;

use crate::{
    day01::{sum_correct_purified_data, sum_purified_data},
    day02::{get_possible_games, get_power_games},
    day03::{sum_gear_ratios, sum_parts},
    day04::{count_total_cards, get_winning_points},
    day05::{get_lowest_seed_location, get_lowest_seed_location_from_ranges},
    day06::{count_ways_to_win, count_ways_to_win_if_one_race},
    day07_v2::{get_total_winnings, get_total_winnings_with_joker},
    day08::{get_ghost_path_length, get_path_length},
    day09::{oasis_value_extrapolator, reverse_oasis_value_extrapolator},
    day10::{count_tiles_enclosed_by_loop, find_loop_length},
    day11::{sum_min_dists, sum_min_dists_if_galaxy_much_older},
    day12::{possible_configs_of_broken_stuff, possible_configs_of_copied_broken_stuff},
    day13::{evaluate_point_of_incidence, evaluate_point_of_incidence_after_removing_smudge},
    day14::{
        calculate_load_on_north_beams_after_cycles, calculate_load_on_north_beams_after_tilting,
    },
    day15::{get_power_of_lenses_after_creating_hashmap, sum_holiday_hash_for_testing_purposes},
    day16_v2::{count_energized_tiles, count_max_energized_tiles},
    day17::{minimize_crucible_temperature_loss, minimize_stable_crucible_temperature_loss},
    day18::{colorize_larger_perimeter, colorize_perimeter},
    day19::{count_good_part_ids, sum_good_part_ids},
    day20::{count_low_and_high_pulse_combinations, get_to_rx},
    day21::{step_counter_for_chad_elf, step_counter_for_elf},
    day22::count_destructable_blocks,
    day23::{get_longest_hike_route_length, get_longest_hike_route_length_without_slopes},
    day24::count_intersections_of_hailstone_trajectories,
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
        "16 1" => println!("{}", count_energized_tiles(&f_cont)),
        "16 2" => println!("{}", count_max_energized_tiles(&f_cont)),
        "17 1" => println!("{}", minimize_crucible_temperature_loss(&f_cont)),
        "17 2" => println!("{}", minimize_stable_crucible_temperature_loss(&f_cont)),
        "18 1" => println!("{}", colorize_perimeter(&f_cont)),
        "18 2" => println!("{}", colorize_larger_perimeter(&f_cont)),
        "19 1" => println!("{}", sum_good_part_ids(&f_cont)),
        "19 2" => println!("{}", count_good_part_ids(&f_cont)),
        "20 1" => println!("{}", count_low_and_high_pulse_combinations(&f_cont)),
        "20 2" => println!("{}", get_to_rx(&f_cont)),
        "21 1" => println!("{}", step_counter_for_elf(&f_cont)),
        "21 2" => println!("{}", step_counter_for_chad_elf(&f_cont)),
        "22 1" => println!("{}", count_destructable_blocks(&f_cont)),
        "23 1" => println!("{}", get_longest_hike_route_length(&f_cont)),
        "23 2" => println!("{}", get_longest_hike_route_length_without_slopes(&f_cont)),
        "24 1" => println!("{}", count_intersections_of_hailstone_trajectories(&f_cont)),
        _ => unreachable!(),
    }

    println!("{:#?}", t.elapsed());
    Ok(())
}
