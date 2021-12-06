use anyhow::Result;
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    solve_1(&input);
    solve_2(&input);

    Ok(())
}

fn solve_1(input: &str) {
    let input = parse_input(input);

    let mut prev_depth : Option<u32> = None;
    let mut n_increase = 0;
    for &depth in input.iter().skip(1) {
        match prev_depth {
            None => {
                prev_depth = depth.into();
            },
            Some(d) => {
                if depth > d {
                    n_increase += 1;
                }
                prev_depth = depth.into();
            }
        }
    }

    println!("{}", n_increase);
}

fn solve_2(input: &str) {
    let input = parse_input(input);


    let mut prev_sum: Option<u32> = None;
    let mut n_increase = 0;
    for depths in input.windows(3) {
        let sum: u32 = depths.iter().sum();
        match prev_sum {
            None => {
                prev_sum = sum.into();
            },
            Some(s) => {
                if sum > s {
                    n_increase += 1;
                }
                prev_sum = sum.into();
            }
        }
    }

    println!("{}", n_increase);
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect()
}
