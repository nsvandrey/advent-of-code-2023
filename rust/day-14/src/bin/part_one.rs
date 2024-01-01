fn main() {
    let input = include_str!("input.txt");
    let output = part_one(input);
    dbg!(output);
}

fn part_one(input: &str) -> usize {
    let mut dish: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    move_rocks(&mut dish);

    let output: usize = dish
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, line)| {
            let rock_count = line.iter().filter(|char| *char == &'O').count();

            (idx + 1) * rock_count
        })
        .sum();

    output
}

fn move_rocks(dish: &mut Vec<Vec<char>>) {
    loop {
        let mut swap_count = 0;
        let mut dish_iterator = dish.iter_mut();
        let mut top_row = dish_iterator.next().unwrap();

        loop {
            match dish_iterator.next() {
                Some(vec) => {
                    let bottom_row = vec;

                    for (top_val, bottom_val) in top_row.iter_mut().zip(bottom_row.iter_mut()) {
                        if top_val == &'.' && bottom_val == &'O' {
                            *top_val = 'O';
                            *bottom_val = '.';
                            swap_count += 1;
                        }
                    }

                    top_row = bottom_row;
                }
                None => break,
            }
        }

        match swap_count {
            0 => break,
            _ => continue,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        );
        assert_eq!(result, 136)
    }
}
