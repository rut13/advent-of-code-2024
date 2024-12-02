use std::env;
use std::fs;

fn get_input(path: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for line in fs::read_to_string(path).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let arg = args.last();
    match arg {
        Some(arg) => {
            let input = get_input(&format!("./data/input-{}.txt", arg));
            println!("{:?}", input);
        },
        None => panic!("No argument provided"), 
    }
}