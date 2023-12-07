fn main() {
    let input = include_str!("input.txt");
    let output = part_two(input);
    dbg!(output);
}

fn part_two(input: &str) -> u64 {
    let mut input_lines = input.lines();
    let race_time = input_lines
        .next()
        .unwrap()
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .chars()
        .filter(|char| !char.is_whitespace())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let distance = input_lines
        .next()
        .unwrap()
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .chars()
        .filter(|char| !char.is_whitespace())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let result = get_race_result(race_time, distance);

    result
}

fn get_race_result(race_time: u64, distance: u64) -> u64 {
    let result = (0..race_time)
        .filter(|time| (race_time - time) * time > distance)
        .count() as u64;
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_two() {
        let result = part_two(
            "Time:      7  15   30
Distance:  9  40  200",
        );
        assert_eq!(result, 71503)
    }
}
