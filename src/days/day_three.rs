use std::fs;

use crate::structs::problem::Problem;
use regex::Regex;
pub struct DayThree {}

impl Problem for DayThree {
    fn part_one(&self, input: Vec<String>) -> String {
        let hay = "]@;why()]&where()@select()mul(589,854)${ <-}$how()^#mul(517,928)^(%@#who()@'mul(82,659):don't()mul(670,226)when(626,911)from()&%{%where())-mul(244,869)<]";
        let re = Regex::new(r"/mul\((\d+),(\d+)\)/").unwrap();
        let dates: Vec<(&str, &str)> = re.captures_iter(hay).map(|caps| {
            let (_, [first, second]) = caps.extract();
            (first, second)
        }).collect();

        println!("{:?}", dates);
        "".to_string()
    }

    fn part_two(&self, input: Vec<String>) -> String {
        todo!()
    }
}