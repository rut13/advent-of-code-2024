use crate::{structs::problem::Problem, utils::parse_input::input_to_vec};
use std::convert::TryInto;

pub struct DayFour {}

fn find_nearby_char(
    character: char,
    (row, column): (isize, isize),
    characters: &Vec<Vec<char>>,
) -> Vec<(isize, isize)> {
    let directions: Vec<(isize, isize)> = vec![
        (-1, -1), // top left
        (0, -1),  // top
        (1, -1),  // top right
        (-1, 0),  // left
        (1, 0),   // right
        (-1, 1),  // bottom left
        (0, 1),   // bottom
        (1, 1),   // bottom right
    ];
    let mut results = Vec::new();
    for (dx, dy) in directions {
        let nx = row + dx;
        let ny = column + dy;

        
        if nx >= 0
            && ny >= 0
            && nx < characters.len().try_into().unwrap()
            && ny < characters[nx as usize].len().try_into().unwrap()
            && characters[nx as usize][ny as usize] == character
        {
            results.push((nx, ny));
        }
    }

    results
}

fn to_multid(input: &Vec<String>) -> Vec<Vec<char>> {
    input.iter().map(|str| str.chars().collect()).collect()
}

impl Problem for DayFour {
    fn part_one(&self, input: String) -> String {
        let parsed_input = input_to_vec(input);
        let multi = to_multid(&parsed_input);
        let mut sum = 0;
        for row in 0..multi.len() {
            for column in 0..multi[row].len() {
                if multi[row][column] == 'X' {
                    println!("Found X x{} y{}", row, column);
                    let m_captures =
                        find_nearby_char('M', (row.try_into().unwrap(), column.try_into().unwrap()), &multi);
                        for (m_x, m_y) in m_captures {
                            let a_captures = find_nearby_char('A', (m_x, m_y), &multi);
                                for (a_x, a_y) in a_captures {
                                    let s_captures = find_nearby_char('S', (a_x, a_y), &multi);
                                    if s_captures.len() > 0 {
                                        sum += 1;
                                    }
                                }
                        }
                }
            }
        }

        sum.to_string()
    }

    fn part_two(&self, _: String) -> String {
        todo!()
    }
}
