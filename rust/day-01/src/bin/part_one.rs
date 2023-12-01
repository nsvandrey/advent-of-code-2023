fn main() {
    let input = include_str!("./input.txt");
    let output = part_one(&input);
    dbg!(output);
}

fn part_one(input: &str) -> u32 {
    let output = input
        .lines()
        .map(|line| {
            parse_line(line)
        })
        .sum::<u32>();

    output
}

fn parse_line(line: &str) -> u32 {
    let mut digits = line
        .chars()
        .filter(|char| {
            char.is_ascii_digit()
        });
    
    let first = digits.next().unwrap();
    let last = match digits.last() {
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
    fn test_part_one() {
        let result = part_one("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet");
        assert_eq!(result, 142)
    }

}