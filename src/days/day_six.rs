use crate::{structs::problem::Problem, utils::parse_input::input_to_vec};

pub struct DaySix {}

#[derive(Debug, Clone)]
struct Position {
    x: i32,
    y: i32
}

#[derive(Debug)]
enum Direction {
    UP,
    RIGHT,
    LEFT,
    DOWN
}

#[derive(Debug)]
struct Guard {
    pos: Position,
    dir: Direction
}

impl Guard {
    pub fn take_step(&mut self, board: &Vec<Vec<char>>) -> Position {
        if !self.can_step(&board) {
            self.rotate_right();
            println!("{:?}", &self.pos);
            return self.pos.clone();
        }
        match self.dir {
            Direction::UP => self.pos.y -= 1,
            Direction::DOWN => self.pos.y += 1,
            Direction::RIGHT => self.pos.x += 1,
            Direction::LEFT => self.pos.x -= 1,
        }
        println!("{:?}", &self.pos);
        self.pos.clone()
    }

    pub fn is_on_board(&self, board: &Vec<Vec<char>>) -> bool {
        self.pos.x >= 0 && self.pos.x < board.len() as i32 && self.pos.y >= 0 && self.pos.y < board[0].len() as i32
    }

    fn rotate_right(&mut self) {
        match self.dir {
            Direction::UP => self.dir = Direction::RIGHT,
            Direction::RIGHT => self.dir = Direction::DOWN,
            Direction::DOWN => self.dir = Direction::LEFT,
            Direction::LEFT => self.dir = Direction::UP,
        }
    }

    fn can_step(&self, board: &Vec<Vec<char>>) -> bool {
        match self.dir {
            Direction::UP => {
                if self.pos.y - 1 < 0 {
                    return true;
                }
                !self.is_obstacle(board[self.pos.y as usize - 1][self.pos.x as usize])
            }
            Direction::DOWN => {
                if self.pos.y + 1 > board[0].len() as i32 - 1 {
                    return true;
                }
                !self.is_obstacle(board[self.pos.y as usize + 1][self.pos.x as usize])
            }
            Direction::LEFT => {
                if self.pos.x - 1 < 0 {
                    return true;
                }
                !self.is_obstacle(board[self.pos.y as usize][self.pos.x as usize - 1])
            }
            Direction::RIGHT => {
                if self.pos.x + 1 > board.len() as i32 - 1 {
                    return true;
                }
                !self.is_obstacle(board[self.pos.y as usize][self.pos.x as usize + 1])
            }
        }

    }

    fn is_obstacle(&self, character: char) -> bool {
        character == '#'
    }

}

impl Problem for DaySix {
    fn part_one(&self, input: String) -> String {
        let parsed_input = input_to_vec(input);
        let mut board: Vec<Vec<char>> = Vec::new();
        let mut start_position: Position = Position { x: 0, y: 0};
        for (i, line) in parsed_input.iter().enumerate() {
            let mut row: Vec<char> = Vec::new();
            for (j, mut character) in line.chars().enumerate() {
                if character == '^' {
                    start_position = Position { x: j as i32, y: i as i32};
                    character = '.';
                }
                row.push(character);
            }
            board.push(row);
        }

        let mut visited: Vec<Position> = Vec::new();
        let mut guard = Guard { dir: Direction::UP, pos: start_position};

        while guard.is_on_board(&board) {
            let new_position = guard.take_step(&board);
            if !visited.iter().any(|Position {x, y}| x == &new_position.x && y == &new_position.y) {
                visited.push(new_position);
            }
        }

        visited.len().to_string()
    }

    fn part_two(&self, _: String) -> String {
        todo!()
    }
}