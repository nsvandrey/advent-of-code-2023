use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let output = part_two(input);
    dbg!(output);
}

fn part_two(input: &str) -> usize {
    let mut hashmap: HashMap<u32, HashMap<&str, (usize, usize)>> = HashMap::new();
    for (idx, step) in input.split(",").enumerate() {
        let mut split = step.split(&['=', '-']);
        let lens = split.next().unwrap();
        let hash = get_hash(lens);
        if step.contains('=') {
            let focal_length = split.next().unwrap().parse::<usize>().unwrap();
            hashmap
                .entry(hash)
                .or_insert(HashMap::new())
                .entry(lens)
                .and_modify(|(_, val)| {
                    *val = focal_length;
                })
                .or_insert((idx, focal_length));
        } else {
            hashmap.entry(hash).or_insert(HashMap::new()).remove(lens);
        }
    }

    let result = hashmap
        .iter()
        .map(|(key, value)| {
            let box_num = *key as usize + 1;
            let mut focal_lengths = value.clone().into_values().collect::<Vec<(usize, usize)>>();

            focal_lengths.sort();

            focal_lengths
                .iter()
                .enumerate()
                .map(|(idx, (_, focal_length))| box_num * (idx + 1) * focal_length)
                .sum::<usize>()
        })
        .sum();

    result
}

fn get_hash(lens: &str) -> u32 {
    let mut current_value = 0;
    for c in lens.chars() {
        current_value += c as u32;
        current_value = current_value * 17;
        current_value = current_value % 256;
    }

    current_value
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_two() {
        let result = part_two("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        assert_eq!(result, 145)
    }
}
