use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let output = part_two(input);
    dbg!(output);
}

fn part_two(input: &str) -> usize {
    let mut input_iterator = input.lines();

    let instructions = input_iterator.next().unwrap().chars();
    let route_map: HashMap<&str, (&str, &str)> = input_iterator
        .skip(1)
        .map(|elem| elem.split_at(3))
        .map(|(key, val)| (key, val[4..12].split_at(3)))
        .map(|(key, (val_a, val_b))| (key, (val_a, &val_b[2..])))
        .collect();

    let start_nodes: Vec<&str> = route_map
        .keys()
        .filter(|key| key.ends_with("A"))
        .cloned()
        .collect();

    let step_counts = start_nodes
        .iter()
        .map(|node| {
            let mut current_node = *node;
            let mut steps = 0;

            instructions
                .clone()
                .cycle()
                .find_map(|instruction| {
                    let next_node = match instruction {
                        'L' => &route_map.get(current_node).unwrap().0,
                        'R' => &route_map.get(current_node).unwrap().1,
                        _ => panic!("Here there be dragons!"),
                    };

                    steps += 1;

                    if next_node.ends_with("Z") {
                        Some(steps)
                    } else {
                        current_node = next_node;
                        None
                    }
                })
                .unwrap()
        })
        .collect::<Vec<usize>>();

    lcm(&step_counts)
}

//lcm implementation stolen from here: https://www.andyloree.com/blog/2022/12/11/least-common-multiple-vect-rust/
fn lcm(numbers: &Vec<usize>) -> usize {
    let mut temp = numbers.clone();

    // check all the same
    loop {
        let mut same = true;

        for idx in 1..temp.len() {
            if temp[0] != temp[idx] {
                same = false;
                break;
            }
        }

        if same {
            return temp[0];
        }

        // Find lowest index
        match temp
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(index, _)| index)
        {
            Some(idx) => {
                temp[idx] = temp[idx] + numbers[idx];
            }
            None => panic!("Not possible"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_two() {
        let result = part_two(
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        );
        assert_eq!(result, 6)
    }
}
