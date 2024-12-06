use crate::structs::problem::Problem;
use regex::Regex;
pub struct DayThree {}

impl Problem for DayThree {
    fn part_one(&self, input: Vec<String>) -> String {
        let re = Regex::new(r"/mul\((\d+),(\d+)\)/").unwrap();
        let merged = input.clone().join("");
        let mults = re.captures_iter(&merged);
        // .map(|caps| {
        //     // let (abc, [first, second, third]) = caps.extract();
        //     let first = caps.get(1).map_or("", |m| m.as_str()).to_string();
        //     let second = caps.get(2).map_or("", |m| m.as_str()).to_string();
        //     println!("first{:?}", first);
        //     println!("second{:?}", second);
        //     (first, second)
        // }).collect();

        println!("{:?}", mults);
        "".to_string()
    }

    fn part_two(&self, _input: Vec<String>) -> String {
        todo!()
    }
}