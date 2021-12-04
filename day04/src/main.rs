use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    value: u8,
    marked: bool,
}

impl Cell {
    pub fn new() -> Self {
        Cell {
            value: 0,
            marked: false,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Board {
    grid: [Cell; Board::HEIGHT * Board::WIDTH],
    is_board_solved: bool,
}

/// Board Index Layout:
/// 00 01 02 03 04
/// 05 06 07 08 09
/// 10 11 12 13 14
/// 15 16 17 18 19
/// 20 21 22 23 24

impl Board {
    const HEIGHT: usize = 5;
    const WIDTH: usize = 5;

    // Create a new, empty board
    pub fn new() -> Self {
        Board {
            grid: [Cell::new(); Board::HEIGHT * Board::WIDTH],
            is_board_solved: false
        }
    }

    // Populate the board give a vector of values
    pub fn populate_board(&mut self, values: &Vec<u8>) {
        for i in 0..Board::HEIGHT * Board::WIDTH {
            self.grid[i].value = values[i];
        }
    }

    // Mark the board if the value exists in the board. If it does, check to
    // see if the board is solved.
    pub fn mark_value(&mut self, value: u8) -> bool {
        for i in 0..self.grid.len() {
            if self.grid[i].value == value {
                self.grid[i].marked = true;
                if self.check_if_board_is_solved(i) && self.is_board_solved == false {
                    println!("\tBoard solved! Score: {}", self.score_board(i));
                    self.is_board_solved = true;
                    return true;
                }
            }
        }
        false
    }

    // Return whether a row or column is complete
    pub fn check_if_board_is_solved(&mut self, index: usize) -> bool {
        self.check_row(index) || self.check_column(index)
    }

    // Checks a row given an index
    fn check_row(&self, index: usize) -> bool {
        let starting_position = index - (index % Board::WIDTH); 
        for i in starting_position..starting_position + Board::WIDTH {
            if self.grid[i].marked == false {
                return false;
            }
        }
        true
    }

    // Checks a column given an index
    fn check_column(&self, index: usize) -> bool {
        let offset = index % Board::HEIGHT;
        for i in 0..Board::HEIGHT {
            let index = i * Board::HEIGHT + offset;
            if self.grid[index].marked == false {
                return false;
            }
        }
        true
    }

    // Gets the score of the board
    fn score_board(&self, index: usize) -> u32 {
        let mut running_score: u32 = 0;
        for i in 0..Board::HEIGHT * Board::WIDTH {
            if self.grid[i].marked == false {
                running_score += self.grid[i].value as u32;
            }
        }
        running_score * self.grid[index].value as u32
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("INPUT: {:?}", &args[1]);

    let (numbers, mut boards) = read_file(&args[1]).unwrap();

    // Part 1:
    println!("Part 1:");
    let mut part_1_boards = boards.clone();
    let part_1_numbers = numbers.clone();
    'outer: for number in part_1_numbers {
        for board in &mut part_1_boards {
            if board.mark_value(number) {
                break 'outer;
            }
        }
    }

    println!("Part 2:");

    // Part 2:
    for number in numbers {
        for board in &mut boards {
            board.mark_value(number);
        }
    }
}

fn read_file<P>(filename: P) -> Result<(Vec<u8>, Vec<Board>), Error>
where
    P: AsRef<Path>,
{
    let drawn_numbers: Vec<u8>;
    let mut boards: Vec<Board> = Vec::new();

    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    // Get the drawn numbers
    let _ = reader.read_line(&mut line)?;

    drawn_numbers = line.trim().split(',').map(|s| s.parse().unwrap()).collect();
    line.clear();

    let mut numbers_in_board: Vec<u8> = Vec::new();

    // Continuously read lines from the files
    while reader.read_line(&mut line)? != 0 {
        if !line.trim().is_empty() {
            // Map the row into a vector, then append it to the larger vector
            let mut row: Vec<u8> = line
                .trim()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            numbers_in_board.append(&mut row);
            // Once we have enough values in the larger vector, put the values
            // onto the board and
            if numbers_in_board.len() == Board::HEIGHT * Board::WIDTH {
                let mut board = Board::new();
                board.populate_board(&numbers_in_board);
                boards.push(board);
                // Clear the larger vector for reuse
                numbers_in_board.clear();
            }
        }
        // Clear the buffer
        line.clear();
    }

    Ok((drawn_numbers, boards))
}
