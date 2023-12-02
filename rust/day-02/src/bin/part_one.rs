const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn main() {
    let input = include_str!("./input.txt");
    let output = part_one(&input);
    dbg!(output);
}

fn part_one(input: &str) -> u32 {
    let output = input.lines().map(|line| parse_line(line)).sum::<u32>();

    output
}

fn parse_line(line: &str) -> u32 {
    let mut line_iterator = line.split(": ");

    let game_id = line_iterator.next().unwrap();
    let records = line_iterator.next().unwrap();

    for record in records.split("; ") {
        if parse_game_record(record) {
            return 0;
        }
    }

    parse_int_from_str(game_id)
}

fn parse_game_record(record: &str) -> bool {
    for cube in record.split(", ") {
        let cube_count = parse_int_from_str(cube);
        if cube.ends_with("red") {
            if cube_count > MAX_RED {
                return true;
            }
        } else if cube.ends_with("green") {
            if cube_count > MAX_GREEN {
                return true;
            }
        } else {
            if cube_count > MAX_BLUE {
                return true;
            }
        }
    }

    false
}

fn parse_int_from_str(str: &str) -> u32 {
    let int = str
        .chars()
        .filter(|char| char.is_ascii_digit())
        .collect::<String>()
        .parse::<u32>()
        .unwrap();

    int
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, 8)
    }
}
