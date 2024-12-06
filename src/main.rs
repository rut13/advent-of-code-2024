use std::env;
use std::fs;

use days::day_one::DayOne;
use days::day_three::DayThree;
use days::day_two::DayTwo;
use structs::problem::Problem;

pub mod days;
pub mod structs;

fn get_input(path: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for line in fs::read_to_string(path).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn day_to_problem(day: usize) -> Option<Box<dyn Problem>> {
    match day {
        1 => Some(Box::new(DayOne {})),
        2 => Some(Box::new(DayTwo {})),
        3 => Some(Box::new(DayThree {})),
        _ => None,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day_arg = args.get(1);
    let day_part_arg = args.get(2);
    match day_arg {
        Some(day_arg) => match day_part_arg {
            Some(day_part_arg) => {
                let input = get_input(&format!("./data/input-{day_arg}.txt"));
                let result = day_to_problem(day_arg.parse::<usize>().unwrap());
                match result {
                    Some(problem) => match day_part_arg.parse::<usize>().unwrap() {
                        1 => println!("{}", problem.part_one(input)),
                        2 => println!("{}", problem.part_two(input)),
                        _ => panic!("Invalid part of day"),
                    },
                    None => panic!("Invalid day"),
                }
            }
            None => panic!("No argument provided for part of day"),
        },
        None => panic!("No argument provided for day"),
    }
}
