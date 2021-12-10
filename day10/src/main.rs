use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mapping = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);

    println!("INPUT: {:?}", &args[1]);

    let lines = read_file(&args[1]).unwrap();

    let mut part_1_running_total: u32 = 0;
    let mut part_2_scores: Vec<u64> = Vec::new();

    for line in lines {
        let mut stack: Vec<char> = Vec::new();
        for c in line.chars() {
            if c == '(' || c == '{' || c == '[' || c == '<' {
                stack.push(c);
            } else {
                if let Some(popped_value) = stack.pop() {
                    let expected_value = mapping[&popped_value];
                    if expected_value != c {
                        part_1_running_total += match c {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => unreachable!("Invalid character!"),
                        };
                        stack.clear();
                        break;
                    }
                }
            }
        }
        if !stack.is_empty() {
            let mut total: u64 = 0;
            while !stack.is_empty() {
                if let Some(popped_value) = stack.pop() {
                    total *= 5;
                    let expected_value = mapping[&popped_value];
                    total += match expected_value {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => unreachable!("Invalid character!"),
                    };
                }
            }
            part_2_scores.push(total);
        }
    }

    part_2_scores.sort();
    let part_2_score = part_2_scores[part_2_scores.len() / 2];

    println!("Part 1: {}", part_1_running_total);
    println!("Part 2: {}", part_2_score);
}

fn read_file<P>(filename: P) -> Result<Vec<String>, Error>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let br = BufReader::new(file);

    Ok(br.lines().map(|line| line.unwrap()).collect())
}
