fn main() {
    let input = include_str!("./input.txt");
    let output = part_two(&input);
    dbg!(output);
}

fn part_two(input: &str) -> u32 {
    let output = input.lines().map(|line| parse_line(line)).sum::<u32>();

    output
}

fn parse_line(line: &str) -> u32 {
    let (mut max_red, mut max_green, mut max_blue) = (0, 0, 0);

    let mut line_iterator = line.split(": ").skip(1);
    let records = line_iterator.next().unwrap();

    for record in records.split("; ") {
        parse_game_record(record, &mut max_red, &mut max_green, &mut max_blue);
    }

    max_red * max_green * max_blue
}

fn parse_game_record(record: &str, red: &mut u32, green: &mut u32, blue: &mut u32) {
    for cube in record.split(", ") {
        let cube_count = parse_int_from_str(cube);
        if cube.ends_with("red") {
            if cube_count > *red {
                *red = cube_count;
            }
        } else if cube.ends_with("green") {
            if cube_count > *green {
                *green = cube_count;
            }
        } else {
            if cube_count > *blue {
                *blue = cube_count;
            }
        }
    }
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
    fn test_part_two() {
        let result = part_two(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, 2286)
    }
}
