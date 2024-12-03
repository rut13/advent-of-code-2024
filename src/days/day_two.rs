use crate::structs::problem::Problem;

pub struct DayTwo {}

impl Problem for DayTwo {
    /**
    *  The levels are either all increasing or all decreasing.
       Any two adjacent levels differ by at least one and at most three.
    */
    fn part_one(&self, input: Vec<String>) -> String {
        let mut result = 0;
        for line in input {
            let split_line: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            let mut increasing = false;
            for i in 0..split_line.len() {
                if i == split_line.len() - 1 {
                    result += 1;
                    break;
                }
                let current = split_line[i];
                let next = split_line[i + 1];
                let diff = i32::abs(current - next);
                if i == 0 {
                    increasing = current < next;
                }
                if diff < 1 || diff > 3 || if !increasing { current < next } else { current > next } {
                    break;
                }
            }
        }
        result.to_string()
    }

    fn part_two(&self, input: Vec<String>) -> String {
        todo!()
    }
}
