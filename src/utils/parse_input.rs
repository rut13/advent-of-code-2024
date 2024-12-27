use std::fs;

pub fn get_input_as_vec(path: &str) -> Vec<String> {
    input_to_vec(parse_input(path))
}

pub fn input_to_vec(input: String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for line in input.lines() {
        result.push(line.to_string())
    }
    result
}

pub fn parse_input(path: &str) -> String {
    let result = fs::read_to_string(path);
    match result {
        Ok(content) => content,
        Err(err) => panic!("Error reading file: {:?}", err),
    }
}