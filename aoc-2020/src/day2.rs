use std::fs;
use std::path::Path;

type PwdConfig<'a> = (usize, usize, &'a str, &'a str);

fn get_input_from_file() -> String {
    let input_path = Path::new("input");
    let input_path = input_path.join("day2");
    let input = fs::read_to_string(input_path).unwrap();
    input
}

fn parse_input(input: &str) -> Vec<PwdConfig> {
    let inputs: Vec<_> = input.trim().split("\n").collect();
    let mut ret = Vec::new();

    for input in inputs {
        let input: Vec<_> = input.trim_end().split(":").collect();
        let rules: Vec<_> = input[0].trim_end().split(" ").collect();
        let rule_count: Vec<_> = rules[0].trim_end().split("-").collect();

        ret.push((
            rule_count[0].parse().unwrap(),
            rule_count[1].parse().unwrap(),
            rules[1],
            input[1].trim(),
        ))
    }

    ret
}

fn part1(input: &str) -> usize {
    let inputs: Vec<PwdConfig> = parse_input(input);
    let mut ret: usize = 0;
    for input in inputs {
        let count: usize = input.3.matches(input.2).count() as usize;

        if input.0 <= count && count <= input.1 {
            ret = ret + 1;
        }
    }
    ret
}

fn part2(input: &str) -> usize {
    let inputs = parse_input(input);
    let mut ret: usize = 0;
    for input in inputs {
        let indices: Vec<_> = input.3.match_indices(input.2).map(|t| t.0).collect();
        let i1 = input.0 - 1;
        let i2 = input.1 - 1;
        let result: (bool, bool) = (indices.contains(&i1), indices.contains(&i2));

        ret += match result {
            (true, false) | (false, true) => 1,
            (_, _) => 0,
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    #[test]
    fn day2_part1_sample_works() {
        assert_eq!(part1(SAMPLE), 2);
    }

    #[test]
    fn day2_part1_works() {
        let input = get_input_from_file();
        assert_eq!(part1(&input), 467);
    }

    #[test]
    fn day2_part2_sample_works() {
        assert_eq!(part2(SAMPLE), 1)
    }

    #[test]
    fn day2_part2_works() {
        let input = get_input_from_file();
        assert_eq!(part2(&input), 441);
    }
}
