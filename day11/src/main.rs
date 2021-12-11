use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("INPUT: {:?}", &args[1]);

    let mut lines = read_file(&args[1]).unwrap();
    let num_rows = lines.len();
    let num_cols = lines[0].len();
    let mut num_flashes: u32 = 0;
    let mut counter: u32 = 0;
    'outer: loop {
        // First, incremenet each value by 1.
        for line in lines.iter_mut() {
            for element in line.iter_mut() {
                *element += 1;
            }
        }
        // Next, any value > 9 we increment the surrounding cells
        let mut already_flashed: Vec<(usize, usize)> = Vec::new();
        'inner: loop {
            let mut flashed = false;
            for i in 0..num_rows {
                for j in 0..num_cols {
                    if lines[i][j] > 9 && !already_flashed.contains(&(i, j)) {
                        num_flashes += 1;
                        flashed = true;
                        already_flashed.push((i, j));
                        increment_surrounding_cells(&mut lines, i, j);
                    }
                }
            }
            if !flashed {
                break 'inner;
            }
        }
        let mut did_all_flash = true;

        // Lastly, any value over 9 we set to 0.
        for line in lines.iter_mut() {
            for element in line.iter_mut() {
                if *element > 9 {
                    *element = 0;
                } else {
                    did_all_flash = false;
                }
            }
        }

        counter += 1;

        if counter == 100 {
            println!("(Part 1) Num Flashes: {}", num_flashes);
        }

        if did_all_flash {
            println!("(Part 2) All flashed at step {}", counter);
            break 'outer;
        }
    }
}

fn increment_surrounding_cells(grid: &mut Vec<Vec<u32>>, cur_row: usize, cur_col: usize) {
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    let can_go_up = cur_row != 0;
    let can_go_down = cur_row != num_rows - 1;
    let can_go_left = cur_col != 0;
    let can_go_right = cur_col != num_cols - 1;

    if can_go_up {
        grid[cur_row - 1][cur_col] += 1;
    }

    if can_go_up && can_go_right {
        grid[cur_row - 1][cur_col + 1] += 1;
    }

    if can_go_right {
        grid[cur_row][cur_col + 1] += 1;
    }

    if can_go_down && can_go_right {
        grid[cur_row + 1][cur_col + 1] += 1;
    }

    if can_go_down {
        grid[cur_row + 1][cur_col] += 1;
    }
    if can_go_down && can_go_left {
        grid[cur_row + 1][cur_col - 1] += 1;
    }

    if can_go_left {
        grid[cur_row][cur_col - 1] += 1;
    }

    if can_go_up && can_go_left {
        grid[cur_row - 1][cur_col - 1] += 1;
    }
}

fn read_file<P>(filename: P) -> Result<Vec<Vec<u32>>, Error>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let br = BufReader::new(file);

    let mut return_vector: Vec<Vec<u32>> = Vec::new();

    let lines: Vec<String> = br.lines().map(|line| line.unwrap()).collect();

    for line in lines {
        return_vector.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
    }

    Ok(return_vector)
}
