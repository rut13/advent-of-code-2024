use crate::structs::problem::Problem;

pub struct DayTwo {}

fn is_safe(report: &[i32]) -> bool {
    let mut increasing = None;
    for i in 0..report.len() - 1 {
        let current = report[i];
        let next = report[i + 1];
        let diff = (current - next).abs();

        if diff < 1 || diff > 3 {
            return false;
        }

        match increasing {
            None => increasing = Some(current < next),
            Some(true) if current >= next => return false,
            Some(false) if current <= next => return false,
            _ => {}
        }
    }
    true
}

impl Problem for DayTwo {
    fn part_one(&self, input: Vec<String>) -> String {
        let mut result = 0;
        for line in input {
            let levels: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            if is_safe(&levels) {
                result += 1;
            }
        }
        result.to_string()
    }

    fn part_two(&self, input: Vec<String>) -> String {
        let mut result = 0;

        for line in input {
            let levels: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();

            if is_safe(&levels) {
                result += 1;
            } else {
                let mut dampener_safe = false;
                for i in 0..levels.len() {
                    let mut modified_levels = levels.clone();
                    modified_levels.remove(i);
                    if is_safe(&modified_levels) {
                        dampener_safe = true;
                        break;
                    }
                }

                if dampener_safe {
                    result += 1;
                }
            }
        }
        result.to_string()
    }
}
