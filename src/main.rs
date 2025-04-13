use rand::Rng;
use std::io::{self, Write};

// size of the game board
const BOARD_SIZE: usize = 10;

struct Board {
    grid: [[CellState; BOARD_SIZE]; BOARD_SIZE],
    ships: Vec<(usize, usize)>,
}

#[derive(Clone, Copy, PartialEq)]

enum CellState {
    Empty,
    Ship,
    Hit,
    Miss,
}

impl Board {
    fn new() -> Self {
        Board {
            grid: [[CellState::Empty; BOARD_SIZE]; BOARD_SIZE],
            ships: Vec::new(),
        }
    }

    // function to randomly place a ship on the board
    fn place_ship(&mut self, size: usize) {
        let mut rng = rand::thread_rng();

        loop {
            let row = rng.gen_range(0..BOARD_SIZE);
            let col = rng.gen_range(0..BOARD_SIZE);
            let direction = rng.gen::<bool>();

            if self.can_place_ship(row, col, size, direction) {
                for i in 0..size {
                    let (r, c) = if direction {
                        (row, col + 1)
                    } else {
                        (row + i, col)
                    };

                    self.grid[r][c] = CellState::Ship;
                    self.ships.push((r, c))
                }
                break;
            }
        }
    }

    fn can_place_ship(&self, row: usize, col: usize, size: usize, direction: bool) -> bool {
        // check if the ship can be placed on the board
        if direction {
            if col + size > BOARD_SIZE {
                return false;
            }

            for i in 0..size {
                if self.grid[row][col + i] != CellState::Empty {
                    return false;
                }
            }
        } else {
            if row + size > BOARD_SIZE {
                return false;
            }

            for i in 0..size {
                if self.grid[row + i][col] != CellState::Empty {
                    return false;
                }
            }
        }
        true
    }

    // method to fire
    fn fire(&mut self, row: usize, col: usize) -> CellState {
        match self.grid[row][col] {
            CellState::Empty => {
                self.grid[row][col] = CellState::Miss;
                CellState::Miss
            }
            CellState::Ship => {
                self.grid[row][col] = CellState::Hit;
                CellState::Hit
            }
            _ => CellState::Miss,
        }
    }

    // method to display the game board
    fn display(&self, hide_ships: bool) {
        print!(" ");
        for i in 0..BOARD_SIZE {
            print!("{ }", i);
        }
        println!();

        for (i, row) in self.grid.iter().enumerate() {
            print!("{:2}", i);
            for cell in row {
                match cell {
                    CellState::Empty => print!(" \u{25A1} "),
                    CellState::Ship => {
                        if hide_ships {
                            print!("   ");
                        } else {
                            print!(" \u{25A0} ");
                        }
                    }
                    CellState::Hit => print!("\x1b[31m \u{25CF} \x1b[0m"),
                    CellState::Miss => print!("\x1b[36m \u{00B7} \x1b[0m"),
                }
                println!();
            }
        }
    }

    // method for game over
    fn is_game_over(&self) -> bool {
        self.ships
            .iter()
            .all(|&(r, c)| self.grid[r][c] == CellState::Hit)
    }
}

fn main() {
    loop {}
}

fn get_player_input() -> (usize, usize) {
    loop {
        print!("\x1b[1;37mEnter coordinated to fire (row, col): \x1b[0m");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Faild to read line");

        let coordinates: Vec<usize> = input
            .trim()
            .split(',')
            .map(|s| s.trim().parse().expect("Invalid input"))
            .collect();
        if coordinates.len() == 2 && coordinates[0] < BOARD_SIZE && coordinates[1] < BOARD_SIZE {
            return (coordinates[0], coordinates[1]);
        } else {
            println!("\x1b[1;31mInvalid input. Please enter row and column numbers seperated by a comma. \x1b[0m");
        }
    }
}

fn generate_opponent_move() -> () {}
