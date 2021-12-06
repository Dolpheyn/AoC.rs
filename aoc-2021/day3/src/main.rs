use anyhow::Result;
use std::io::{self, Read};
use transpose::transpose;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    solve_1(&input);
    solve_2(&input);

    Ok(())
}

/// Power consumption = gamma rate * epsilon rate
/// gamma = most common bit
/// epsilon = least common bit
fn solve_1(input: &str) {
    let input = parse_input(input);

    let n_row = input.len();
    let n_col = input[0].len();
    let transposed = transpose_input(&input, n_row, n_col);

    let mut epsilon_rate = String::new();
    let mut gamma_rate = String::new();
    for i in 0..n_col {
        let i = i * n_row;
        let bits = &transposed[i..i+n_row];

        let bit_1_count: u32 = bits.iter().sum();
        let bit_0_count: u32 = (bit_1_count as i32 - n_row as i32).abs() as u32;

        if bit_1_count > bit_0_count {
            gamma_rate.push('1');
            epsilon_rate.push('0');
        } else {
            gamma_rate.push('0');
            epsilon_rate.push('1');
        }
    }

    let gamma_rate = usize::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_rate = usize::from_str_radix(&epsilon_rate, 2).unwrap();

    println!("{}", gamma_rate * epsilon_rate);
}

/// Life support rating = oxygen generator rating * CO2 scrubber rating
/// oxygen generator: most common
/// CO2 scrubber: least common
fn solve_2(input: &str) {
    let input = parse_input(input);

    let n_row = input.len();
    let n_col = input[0].len();
    let transposed = transpose_input(&input, n_row, n_col);

    let oxygens = Vec::new();
    let co2s = Vec::new();
    for i in 0..n_col {
        let ptr = i * n_row;
        let bits = &transposed[ptr..ptr+n_row];

        let bit_1_count: u32 = bits.iter().sum();
        let bit_0_count: u32 = (bit_1_count as i32 - n_row as i32).abs() as u32;

        let (most_common_bit, least_common_bit) = if bit_1_count > bit_0_count { ('1', '0') } else { ('0', '1') };

        // TODO: how to filter from str in input according to character in nth column?
    }
}

fn transpose_input(input: &Vec<&str>, n_row: usize, n_col: usize) -> Vec<u32> { 
    let input: Vec<u32> = input.iter()
        .map(|binary| binary.chars())
        .flatten()
        .map(|bit| bit.to_digit(10).unwrap())
        .collect();

    let mut transposed: Vec<u32> = vec![0; n_col * n_row];
    transpose(&input, &mut transposed, n_col, n_row);

    transposed
}

fn parse_input(input: &str) -> Vec<&str> {
    input
        .lines()
        .collect()
}
