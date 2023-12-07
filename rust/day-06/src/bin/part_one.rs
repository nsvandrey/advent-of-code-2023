use std::iter::zip;

fn main() {
    let input = include_str!("input.txt");
    let output = part_one(input);
    dbg!(output);
}

fn part_one(input: &str) -> u32 {
    let mut input_lines = input.lines();
    let race_times = input_lines
        .next()
        .unwrap()
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .split_whitespace()
        .map(|elem| elem.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let distances = input_lines
        .next()
        .unwrap()
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .split_whitespace()
        .map(|elem| elem.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let result = zip(race_times, distances)
        .map(|(race_time, distance)| get_race_result(race_time, distance))
        .product::<u32>();

    result
}

fn get_race_result(race_time: u32, distance: u32) -> u32 {
    let result = (0..race_time)
        .filter(|time| (race_time - time) * time > distance)
        .count() as u32;
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(
            "Time:      7  15   30
Distance:  9  40  200",
        );
        assert_eq!(result, 288)
    }
}
