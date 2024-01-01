fn main() {
    let input = include_str!("input.txt");
    let output = part_one(input);
    dbg!(output);
}

fn part_one(input: &str) -> usize {
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
        if window[0] == window[1] {
            stack.push(idx)
        }
    }

    for idx in stack {
        let line_a = &grid[0..=idx].iter().rev();
        let line_b = &grid[idx + 1..];

        match line_a
            .clone()
            .zip(line_b)
            .all(|(a, b)| a == b)
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
            return output
        },
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

    for (idx, window) in transposed_grid.windows(2).enumerate() {
        if window[0] == window[1] {
            stack.push(idx)
        }
    }

    for idx in stack {
        let line_a = &transposed_grid[0..=idx].iter().rev();
        let line_b = &transposed_grid[idx + 1..];

        match line_a
            .clone()
            .zip(line_b)
            .all(|(a, b)| a == b)
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
    
    output
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(
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
        assert_eq!(result, 405)
    }
}
