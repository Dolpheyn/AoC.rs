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
        .as_slice()
        .windows(2)
        .map(|pair| pair[1] > pair[0])
        .fold(0, |acc, x| if x { acc + 1 } else { acc });

    println!("{}", n_increase);
    assert_eq!(n_increase, 1583);

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
    assert_eq!(n_increase, 1627);
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect()
}
