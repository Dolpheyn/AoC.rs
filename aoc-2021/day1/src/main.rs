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
    let n_increase = input
        .windows(2)
        .map(|pair| pair[1] > pair[0])
        .filter(|b| *b)
        .count();

    println!("{}", n_increase);
    assert_eq!(n_increase, 1583);
}

fn solve_2(input: &str) {
    let input = parse_input(input);

    let sums = input
        .windows(3)
        .map(|w| w.iter().sum())
        .collect::<Vec<u32>>();

    let n_increase = sums
        .windows(2)
        .map(|pair| pair[1] > pair[0])
        .filter(|b| *b)
        .count();

    println!("{}", n_increase);
    assert_eq!(n_increase, 1627);
}

fn parse_input(input: &str) -> Vec<u32> {
    input.lines().map(|l| l.parse::<u32>().unwrap()).collect()
}
