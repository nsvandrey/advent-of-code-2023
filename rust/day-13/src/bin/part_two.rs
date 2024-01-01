fn main() {
    let input = include_str!("input.txt");
    let output = part_two(input);
    dbg!(output);
}

fn part_two(input: &str) -> usize {
    let input_iter = input
        .split("\n\n")
        .map(|grid| grid.lines().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let y: usize = input_iter.iter().map(|grid| parse_grid(&grid)).sum();
    y
}

fn parse_grid(grid: &Vec<&str>) -> usize {
    let grid_iterator = grid.windows(2);
    let mut stack: Vec<usize> = vec![];
    let mut split_idx: Option<usize> = None;
    let mut output = 0;

    for (idx, window) in grid_iterator.enumerate() {
        if window[0]
            .chars()
            .zip(window[1].chars())
            .filter(|(char_a, char_b)| char_a != char_b)
            .count()
            <= 1
        {
            stack.push(idx)
        }
    }

    for idx in stack {
        let line_a = (&grid[0..=idx]).iter().map(|line| line.chars()).rev();
        let line_b = (&grid[idx + 1..]).iter().map(|line| line.chars());

        match (line_a
            .clone()
            .flatten()
            .zip(line_b.flatten())
            .filter(|(a, b)| a != b)
            .count()
            == 1)
            .then_some(idx + 1)
        {
            Some(num) => {
                split_idx = Some(num);
                break;
            }
            None => continue,
        }
    }

    match split_idx {
        Some(num) => {
            output += 100 * num;
            dbg!(&output);
            return output;
        }
        None => output += 0,
    };

    let mut transposed_grid: Vec<Vec<char>> =
        (0..grid[0].len()).map(|_| vec![]).collect::<Vec<_>>();

    for row in grid {
        for (val, transposed_row) in row.chars().zip(&mut transposed_grid) {
            transposed_row.push(val);
        }
    }

    let mut stack: Vec<usize> = vec![];

    let transposed_grid_iterator = transposed_grid.windows(2);

    for (idx, window) in transposed_grid_iterator.enumerate() {
        if window[0]
            .iter()
            .zip(window[1].iter())
            .filter(|(char_a, char_b)| char_a != char_b)
            .count()
            <= 1
        {
            stack.push(idx)
        }
    }

    for idx in stack {
        let line_a = &transposed_grid[0..=idx].iter().rev();
        let line_b = &transposed_grid[idx + 1..].iter();

        match (line_a
            .clone()
            .flatten()
            .zip(line_b.clone().flatten())
            .filter(|(a, b)| a != b)
            .count()
            == 1)
            .then_some(idx + 1)
        {
            Some(num) => {
                split_idx = Some(num);
                break;
            }
            None => continue,
        }
    }

    match split_idx {
        Some(num) => output += num,
        None => output += 0,
    };

    dbg!(&output);
    output
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_two() {
        let result = part_two(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        );
        assert_eq!(result, 400)
    }
}
