use anyhow::Result;
use std::io::{self, Read};

#[derive(Debug)]
enum Command {
    Forward(isize),
    Down(isize),
    Up(isize),
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    solve_1(&input);
    solve_2(&input);

    Ok(())
}

fn solve_1(input: &str) {
    let input = parse_input(input);

    let mut y = 0;
    let mut x = 0;
    for command in input {
        match command {
            Command::Forward(n) => { x += n; }
            Command::Down(n) => { y += n; }
            Command::Up(n) => { y -= n; }
        }
    }

    println!("{}", x * y);
}

fn solve_2(input: &str) {
    let input = parse_input(input);

    let mut aim = 0;
    let mut x = 0;
    let mut y = 0;
    for command in input {
        match command {
            Command::Up(n) => { aim -= n },
            Command::Down(n) => { aim += n },
            Command::Forward(n) => {
                x += n;
                y += n * aim;
            }
        }
    }

    println!("{}", x * y);
}

fn parse_input(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|l| l.split(" ").collect::<Vec<_>>())
        .map(|s: Vec<&str>| [s[0], s[1]].into())
        .collect()
}

impl Into<Command> for [&str; 2] {
    fn into(self) -> Command {
        match self {
            ["forward", n] => Command::Forward(n.parse().unwrap()),
            ["down", n] => Command::Down(n.parse().unwrap()),
            ["up", n] => Command::Up(n.parse().unwrap()),
            _ => panic!("wut")
        }
    }

}
