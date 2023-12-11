use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let output = part_one(input);
    dbg!(output);
}

fn part_one(input: &str) -> u32 {
    let mut input_iterator = input.lines();

    let instructions = input_iterator.next().unwrap().chars();
    let route_map: HashMap<&str, (&str, &str)> = input_iterator
        .skip(1)
        .map(|elem| elem.split_at(3))
        .map(|(key, val)| (key, val[4..12].split_at(3)))
        .map(|(key, (val_a, val_b))| (key, (val_a, &val_b[2..])))
        .collect();

    let mut current_point = &"AAA";
    let mut steps = 0;

    for instruction in instructions.clone().cycle() {
        steps += 1;

        current_point = match instruction {
            'L' => &route_map.get(current_point).unwrap().0,
            'R' => &route_map.get(current_point).unwrap().1,
            _ => panic!("Here there be dragons!"),
        };

        if current_point == &"ZZZ" {
            break;
        }
    }

    steps
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one_a() {
        let result = part_one(
            "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(result, 2)
    }

    #[test]
    fn test_part_one_b() {
        let result = part_one(
            "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(result, 6)
    }
}
