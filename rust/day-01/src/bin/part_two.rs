use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part_two(input);
    dbg!(output);
}

fn part_two(input: &str) -> u32 {
    let output = input.lines().map(|line| parse_line(line)).sum::<u32>();

    output
}

fn parse_line(line: &str) -> u32 {
    let digits: HashMap<&str, u32> = HashMap::from([
        ("one", 1), ("1", 1),
        ("two", 2), ("2", 2),
        ("three", 3), ("3", 3),
        ("four", 4), ("4", 4),
        ("five", 5), ("5", 5),
        ("six", 6), ("6", 6),
        ("seven", 7), ("7", 7),
        ("eight", 8), ("8", 8),
        ("nine", 9), ("9", 9),
    ]);

    let mut line_iterator = (0..line.len()).filter_map(|idx| {
        let substring = &line[idx..];
        let mut val = None;

        for (key, value) in &digits {
            if substring.starts_with(key) {
                val = Some(value);
                break;
            }
        }

        val
    });

    let first = line_iterator.next().unwrap();
    let last = match line_iterator.last() {
        Some(digit) => digit,
        None => first,
    };

    let calibration = format!("{first}{last}");

    calibration.parse::<u32>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two() {
        let result = part_two(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(result, 281)
    }
}
