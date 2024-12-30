use crate::structs::problem::Problem;

pub struct DayFive {}

struct Rule {
    x: u32,
    y: u32,
}

fn parse_sections(input: String) -> (Vec<Rule>, Vec<Vec<u32>>) {
    let split: Vec<&str> = input.split("\n\n").collect();
    let raw_rules = split[0].split("\n");
    let raw_updates = split[1].split("\n");

    let rules: Vec<Rule> = raw_rules
        .map(|rule| {
            let parts: Vec<&str> = rule.split("|").collect();
            let x: u32 = parts[0].parse().unwrap();
            let y: u32 = parts[1].parse().unwrap();
            Rule { x, y }
        })
        .collect();

    let updates: Vec<Vec<u32>> = raw_updates
        .map(|update| update.split(",").map(|num| num.parse().unwrap()).collect())
        .collect();

    (rules, updates)
}

fn rule_exists(rules: &Vec<Rule>, x: u32, y: u32) -> bool {
    for rule in rules {
        if rule.x == x && rule.y == y {
            return true;
        }
    }
    false
}

fn is_valid(rules: &Vec<Rule>, update: &Vec<u32>) -> bool {
    for i in 0..update.len() {
        if i < update.len() - 1 {
            let current = update[i];
            let (_, upcoming) = update.split_at(i + 1);
            for j in 0..upcoming.len() {
                let next = upcoming[j];
                if rule_exists(rules, next, current) {
                    return false;
                }
            }
        }
    }
    true
}

fn sort(rules: &Vec<Rule>, nums: &Vec<u32>) -> Vec<u32> {
    let mut sorted_nums = nums.clone();
    loop {
        for i in 0..sorted_nums.len() {
            if i < sorted_nums.len() - 1 {
                let current = sorted_nums[i];
                let next = sorted_nums[i + 1];
                if rule_exists(rules, next, current) {
                    sorted_nums.swap(i, i + 1);
                }
            }
        }
        if is_valid(&rules, &sorted_nums) {
            return sorted_nums;
        }
    }
}

impl Problem for DayFive {
    fn part_one(&self, input: String) -> String {
        let (rules, updates) = parse_sections(input);
        let mut sum = 0;
        for update in updates {
            if is_valid(&rules, &update) {
                let middle = update.len() / 2;
                sum += update[middle];
            }
        }

        sum.to_string()
    }

    fn part_two(&self, input: String) -> String {
        let (rules, updates) = parse_sections(input);
        let mut sum = 0;
        for update in updates {
            if !is_valid(&rules, &update) {
                let sorted = sort(&rules, &update);
                let middle = sorted.len() / 2;
                sum += sorted[middle];
            }
        }

        sum.to_string()
    }
}
