use std::collections::HashMap;
use std::iter::repeat;

fn main() {
    let input = include_str!("input.txt");
    let output = part_one(input);
    dbg!(output);
}

fn part_one(input: &str) -> usize {
    let output: usize = input.lines().map(|line| parse_line(line)).sum();
    output
}

fn parse_line(line: &str) -> usize {
    let mut permutations: HashMap<(usize, usize), usize> = HashMap::new();
    permutations.insert((0, 0), 1);

    let mut line_iterator = line.split_whitespace();
    let springs = line_iterator.next().unwrap();
    let springs = repeat(springs).take(5).collect::<Vec<&str>>().join(&"?");
    let broken_groups: Vec<usize> = line_iterator
        .next()
        .unwrap()
        .split(',')
        .map(|elem| elem.parse::<usize>().unwrap())
        .collect();

    let broken_groups = repeat(broken_groups)
        .take(5)
        .flatten()
        .collect::<Vec<usize>>();
    dbg!(&broken_groups);

    for spring in springs.chars() {
        let mut next: Vec<(usize, usize, usize)> = vec![];

        for (key, value) in &permutations {
            let group_id = key.0;
            let group_amount = key.1;

            if spring != '#' {
                if group_amount == 0 {
                    next.push((group_id, group_amount, *value))
                } else if group_amount == broken_groups[group_id] {
                    next.push((group_id + 1, 0, *value))
                }
            }

            if spring != '.' {
                if group_id < broken_groups.len() && group_amount < broken_groups[group_id] {
                    next.push((group_id, group_amount + 1, *value))
                }
            }
        }

        permutations.clear();

        for (group_id, group_amount, value) in next {
            let perm_val = permutations.entry((group_id, group_amount)).or_insert(0);
            *perm_val += value;
        }
    }

    let result: usize = permutations
        .iter()
        .filter(|(key, _)| {
            key.0 == broken_groups.len()
                || (key.0 == broken_groups.len() - 1 && key.1 == broken_groups[key.0])
        })
        .map(|(_, value)| value)
        .sum();
    dbg!(&result);
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(
            "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
        );
        assert_eq!(result, 525152)
    }
}
