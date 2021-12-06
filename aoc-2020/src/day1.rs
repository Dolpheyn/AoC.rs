use std::fs;
use std::path::Path;

fn parse_input(input: &str) -> Vec<&str> {
    let input = input.trim_end().split("\n").collect::<Vec<_>>();
    input
}

pub fn part1(input: &str) -> i64 {
    let input = parse_input(input);

    for x in &input {
        for y in &input {
            let x = x.parse::<i64>().unwrap();
            let y = y.parse::<i64>().unwrap();

            if 2020 - x - y == 0 {
                return x * y;
            }
        }
    }
    unreachable!();
}

pub fn part2(input: &str) -> i64 {
    let input = parse_input(input);
    for x in &input {
        for y in &input {
            for z in &input {
                let x = x.parse::<i64>().unwrap();
                let y = y.parse::<i64>().unwrap();
                let z = z.parse::<i64>().unwrap();

                if 2020 - x - y - z == 0 {
                    return x * y * z;
                }
            }
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &str = "1721
979
366
299
675
1456";

    fn get_input_from_file() -> String {
        let input_path = Path::new("input");
        let input_path = input_path.join("day1");
        let input = fs::read_to_string(input_path).unwrap();
        input
    }

    #[test]
    fn day1_part1_sample_works() {
        assert_eq!(part1(SAMPLE), 514579);
    }

    #[test]
    fn day1_part1_works() {
        let input = get_input_from_file();
        assert_eq!(part1(&input), 1006875);
    }

    #[test]
    fn day1_part2_sample_works() {
        assert_eq!(part2(SAMPLE), 241861950);
    }

    #[test]
    fn day1_part2_works() {
        let input = get_input_from_file();
        assert_eq!(part2(&input), 165026160);
    }
}
