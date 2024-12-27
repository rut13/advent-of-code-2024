use crate::{structs::problem::Problem, utils::parse_input::input_to_vec};
use regex::Regex;
pub struct DayThree {}

fn total_sum(mults: &Vec<(u32, u32)>) -> u32 {
    let mut sum = 0;
    for (first, second) in mults {
        sum += first * second;
    }
    sum
}

impl Problem for DayThree {
    fn part_one(&self, input: String) -> String {
        let parsed_input = input_to_vec(input);
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let merged = parsed_input.clone().join("");
        let mults: Vec<(u32, u32)> = re.captures_iter(&merged).filter_map(|caps| {
            let first = caps.get(1)?.as_str().parse::<u32>().ok()?;
            let second = caps.get(2)?.as_str().parse::<u32>().ok()?;
            Some((first, second))
        }).collect();

        total_sum(&mults).to_string()
    }

    fn part_two(&self, input: String) -> String {
        let parsed_input = input_to_vec(input);
        let re = Regex::new(r"mul\((\d+),(\d+)\)|do(?:n't)?\(\)").unwrap();
        let merged = parsed_input.clone().join("");
        let mut allow = None;
        let mults: Vec<(u32, u32)> = re.captures_iter(&merged).filter_map(|caps| {
            if let Some(do_match) = caps.get(0) {
                let first = do_match.as_str();
                if first.starts_with("do(") {
                    allow = Some(true);
                    return None;
                } else if first.starts_with("don't(") {
                    allow = Some(false);
                    return None;
                }
            }

            if let (Some(first_num), Some(second_num)) = (caps.get(1), caps.get(2)) {
                match allow {
                    Some(true) | None => {
                        let first = first_num.as_str().parse::<u32>().ok()?;
                        let second = second_num.as_str().parse::<u32>().ok()?;
                        return Some((first, second));
                    }
                    _ => return None,
                }
            }

            None
        }).collect();



        total_sum(&mults).to_string()
    }
}