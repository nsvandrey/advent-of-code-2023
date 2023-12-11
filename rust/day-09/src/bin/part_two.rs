fn main() {
    let input = include_str!("input.txt");
    let output = part_two(input);
    dbg!(output);
}

fn part_two(input: &str) -> isize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|elem| elem.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .map(|line_vec| parse_line(line_vec))
        .sum::<isize>()
}

fn parse_line(line: Vec<isize>) -> isize {
    let mut stack: Vec<Vec<isize>> = vec![];
    let mut line = line;
    stack.push(line.clone());

    while !is_zero_vec(&line) {
        let extrapolate: Vec<isize> = line.windows(2).map(|slice| slice[1] - slice[0]).collect();
        stack.push(extrapolate.clone());
        line = extrapolate;
    }

    let mut add_val = 0;

    loop {
        match stack.pop() {
            Some(history) => add_val = history.iter().next().unwrap() - add_val,
            _ => break,
        }
    }

    add_val
}

fn is_zero_vec(vec: &[isize]) -> bool {
    !vec.iter().any(|&elem| elem != 0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_two() {
        let result = part_two("10 13 16 21 30 45");
        assert_eq!(result, 5)
    }
}
