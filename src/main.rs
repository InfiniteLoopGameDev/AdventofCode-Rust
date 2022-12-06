use std::env;

pub mod year2022;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].parse::<i32>().unwrap() {
        2022 => match args[2].parse::<i32>().unwrap() {
            1 => year2022::day1::solution(),
            2 => year2022::day2::solution(),
            3 => year2022::day3::solution(),
            4 => year2022::day4::solution(),
            5 => year2022::day5::solution(),
            6 => year2022::day6::solution(),
            _ => panic!("Invalid argument input"),
        },
        _ => panic!("Invalid argument input"),
    }
}
