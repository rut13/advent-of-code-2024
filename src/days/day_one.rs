use crate::structs::problem::Problem;

pub struct DayOne {}

fn parse_and_sort(input: Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut left_result: Vec<i32> = Vec::new();
    let mut right_result: Vec<i32> = Vec::new();
    for line in input {
        let split_line: Vec<&str> = line.split_whitespace().collect();
        let first = split_line.first().unwrap();
        let last = split_line.last().unwrap();
        left_result.push(first.parse::<i32>().unwrap());
        right_result.push(last.parse::<i32>().unwrap());
    }
    left_result.sort();
    right_result.sort();
    (left_result, right_result)
}

impl Problem for DayOne {
    fn part_one(&self, input: Vec<String>) -> String {
        let (left, right) = parse_and_sort(input);
        let mut sum = 0;
        for i in 0..left.len() {
            let diff = right[i] - left[i];
            sum += i32::abs(diff);
        }
        sum.to_string()
    }

    fn part_two(&self, input: Vec<String>) -> String {
        todo!()
    }
}