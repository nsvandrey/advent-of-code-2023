fn main() {
    let input = include_str!("input.txt");
    let output = part_one(input);
    dbg!(output);
}

fn part_one(input: &str) -> u32 {
    let mut points = 0;

    for line in input.lines() {
        let winning_count = parse_line(line);
        if winning_count == 0 {
            continue;
        } else {
            let base: u32 = 2;
            points += base.pow(winning_count - 1);
        }
    }

    points
}

fn parse_line(line: &str) -> u32 {
    let line_iterator = line.split(": ").skip(1).collect::<String>();
    let mut line_iterator = line_iterator.split(" | ");

    let winning_numbers = line_iterator.next().unwrap();
    let scratch_numbers = line_iterator.next().unwrap();

    let winning_numbers = winning_numbers
        .split_whitespace()
        .map(|elem| elem.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let intersect = scratch_numbers
        .split_whitespace()
        .map(|elem| elem.parse::<u32>().unwrap())
        .filter(|num| winning_numbers.contains(&num))
        .collect::<Vec<u32>>();

    intersect.len() as u32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, 13)
    }
}
