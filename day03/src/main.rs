use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::Path;

const ARRAY_SIZE: usize = 12;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("INPUT: {:?}", &args[1]);

    let data = read_lines(&args[1]).unwrap();

    println!(
        "(Part 1) Power Consumption: {}",
        get_power_consumption(&data)
    );

    println!(
        "(Part 2) Life Support Rating: {}",
        get_life_support_rating(&data)
    );
}

fn read_lines<P>(filename: P) -> Result<Vec<u32>, Error>
where
    P: AsRef<Path>,
{
    let br = BufReader::new(File::open(filename)?);
    br.lines()
        .map(|line| line.and_then(|v| u32::from_str_radix(v.as_ref(), 2).map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn get_power_consumption(array: &Vec<u32>) -> u32 {
    let mut counter_per_index: [i32; ARRAY_SIZE] = [0; ARRAY_SIZE];

    for value in array {
        for i in 0..ARRAY_SIZE {
            if (value >> i & 0x1) == 1 {
                counter_per_index[ARRAY_SIZE - 1 - i] += 1;
            } else {
                counter_per_index[ARRAY_SIZE - 1 - i] -= 1;
            }
        }
    }

    let mut gamma_rate: u32 = 0;

    for count in counter_per_index {
        if count > 0 {
            gamma_rate += 1;
        }
        gamma_rate = gamma_rate << 1;
    }

    gamma_rate = gamma_rate >> 1;

    let epsilon_rate = !(gamma_rate & 0xfff) & 0xfff;

    gamma_rate * epsilon_rate
}

fn get_life_support_mask(array: &Vec<u32>, use_least_common: bool) -> u32 {
    let mut mask: u32 = 0;
    let mut number_of_matches: u32 = 0;
    let mut last_value: u32 = 0;
    for i in 0..ARRAY_SIZE {
        let mut value_counter = 0;
        for value in array {

            let value_to_compare = value >> (ARRAY_SIZE - i);
            let current_mask = mask >> (ARRAY_SIZE - i);

            if value_to_compare != current_mask && i != 0 {
                continue;
            }

            last_value = *value;
            number_of_matches += 1;

            if (value >> (ARRAY_SIZE - 1 - i) & 0x1) == 1 {
                value_counter += 1;
            }
            else {
                value_counter -= 1;
            }
        }

        if number_of_matches == 1 {
            mask = last_value;
            break;
        }

        number_of_matches = 0;

        if value_counter >= 0 && use_least_common == false {
            mask |= 1 << (ARRAY_SIZE - 1 - i);
        }
        else if value_counter < 0 && use_least_common == true {
            mask |= 1 << (ARRAY_SIZE - 1 - i);
        }
    }
    mask
}

fn get_life_support_rating(array: &Vec<u32>) -> u32 {

    let oxygen_generator_rating = get_life_support_mask(&array, false);
    
    let co2_scrubber_rating = get_life_support_mask(&array, true);

    oxygen_generator_rating * co2_scrubber_rating
}

// 400929 too low