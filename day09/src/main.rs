use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;

struct Cave {
    grid: Vec<Vec<u32>>,
    marked: Vec<Vec<bool>>,
}

impl Cave {
    fn set_position_as_marked(&mut self, row_pos: usize, col_pos: usize) {
        self.marked[row_pos][col_pos] = true;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("INPUT: {:?}", &args[1]);

    let lines = read_file(&args[1]).unwrap();

    let num_rows = lines.len();
    let num_cols = lines[0].len();

    let mut running_total: u32 = 0;
    for i in 0..num_rows {
        for j in 0..num_cols {
            let current_value = lines[i][j];
            let mut smallest_value = true;

            if j != 0 {
                if lines[i][j - 1] <= current_value {
                    smallest_value = false;
                }
            }
            if j != num_cols - 1 {
                if lines[i][j + 1] <= current_value {
                    smallest_value = false;
                }
            }
            if i != 0 {
                if lines[i - 1][j] <= current_value {
                    smallest_value = false;
                }
            }
            if i != num_rows - 1 {
                if lines[i + 1][j] <= current_value {
                    smallest_value = false;
                }
            }
            if smallest_value {
                running_total += current_value + 1;
            }
        }
    }

    println!("Part 1: {}", running_total);

    let mut return_values: Vec<u32> = Vec::new();
    for i in 0..num_rows {
        for j in 0..num_cols {
            let current_value = lines[i][j];

            // println!("Row: {}, Evaluating {}", i, current_value);
            let mut cave = Cave {
                grid: lines.clone(),
                marked: vec![vec![false; num_cols]; num_rows],
            };
            let value = find_basin_size(&mut cave, current_value, i, j);
            if value > 0 {
                return_values.push(value);
            }
        }
    }

    return_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut return_value = 1;
    return_values
        .iter_mut()
        .rev()
        .take(3)
        .for_each(|x| return_value *= *x);
    println!("Part 2: {:?}", return_value);
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

fn find_basin_size(cave: &mut Cave, starting_value: u32, row_pos: usize, col_pos: usize) -> u32 {
    let num_rows = cave.grid.len();
    let num_cols = cave.grid[0].len();
    let mut return_value = 1;
    let current_value = cave.grid[row_pos][col_pos];

    if cave.marked[row_pos][col_pos] {
        return 0;
    }

    cave.set_position_as_marked(row_pos, col_pos);

    // First, make sure there is nothing smaller or equal than the original starting value
    if starting_value == 9 {
        return 0;
    }
    if current_value == starting_value {
        if col_pos != 0 {
            if cave.grid[row_pos][col_pos - 1] <= starting_value {
                return 0;
            }
        } else if col_pos != num_cols - 1 {
            if cave.grid[row_pos][col_pos + 1] <= starting_value {
                return 0;
            }
        }
        if row_pos != 0 {
            if cave.grid[row_pos - 1][col_pos] <= starting_value {
                return 0;
            }
        }
        if row_pos != num_rows - 1 {
            if cave.grid[row_pos + 1][col_pos] <= starting_value {
                return 0;
            }
        }
    }

    // Second, get the size
    if col_pos != 0 {
        if cave.grid[row_pos][col_pos - 1] > current_value && cave.grid[row_pos][col_pos - 1] != 9 {
            return_value += find_basin_size(cave, starting_value, row_pos, col_pos - 1);
        }
    }
    if col_pos != num_cols - 1 {
        if cave.grid[row_pos][col_pos + 1] > current_value && cave.grid[row_pos][col_pos + 1] != 9 {
            return_value += find_basin_size(cave, starting_value, row_pos, col_pos + 1);
        }
    }
    if row_pos != 0 {
        if cave.grid[row_pos - 1][col_pos] > current_value && cave.grid[row_pos - 1][col_pos] != 9 {
            return_value += find_basin_size(cave, starting_value, row_pos - 1, col_pos)
        }
    }
    if row_pos != num_rows - 1 {
        if cave.grid[row_pos + 1][col_pos] > current_value && cave.grid[row_pos + 1][col_pos] != 9 {
            return_value += find_basin_size(cave, starting_value, row_pos + 1, col_pos);
        }
    }

    return_value
}
