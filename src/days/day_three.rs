use crate::structs::problem::Problem;
use regex::Regex;
pub struct DayThree {}

impl Problem for DayThree {
    fn part_one(&self, input: Vec<String>) -> String {
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let merged = input.clone().join("");
        let mults: Vec<(u32, u32)> = re.captures_iter(&merged).filter_map(|caps| {
            let first = caps.get(1)?.as_str().parse::<u32>().ok()?;
            let second = caps.get(2)?.as_str().parse::<u32>().ok()?;
            Some((first, second))
        }).collect();

        let mut sum = 0;
        for (first, second) in mults {
            sum += first * second;
        }

        sum.to_string()
    }

    fn part_two(&self, _input: Vec<String>) -> String {
        todo!()
    }
}