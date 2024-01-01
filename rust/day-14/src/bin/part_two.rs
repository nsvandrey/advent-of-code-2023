use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let output = part_two(input);
    dbg!(output);
}

fn part_two(input: &str) -> usize {
    let mut dish: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut permutations: HashSet<Vec<Vec<char>>> = HashSet::new();
    let iterations = 1000000000;

    for _ in 0..iterations {
        cycle_rocks(&mut dish);
        match permutations.insert(dish.clone()) {
            true => continue,
            false => break,
        }
    }

    let snapshot = permutations.len();
    permutations.clear();
    permutations.insert(dish.clone());

    for _ in 0..(iterations - snapshot) {
        cycle_rocks(&mut dish);
        match permutations.insert(dish.clone()) {
            true => continue,
            false => break,
        }
    }

    let remaining_cycles = (iterations - snapshot) % permutations.len() - 1;

    for _ in 0..remaining_cycles {
        cycle_rocks(&mut dish);
    }

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

fn cycle_rocks(dish: &mut Vec<Vec<char>>) {
    move_rocks_north(dish);
    move_rocks_west(dish);
    move_rocks_south(dish);
    move_rocks_east(dish);
}

fn move_rocks_north(dish: &mut Vec<Vec<char>>) {
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

fn move_rocks_west(dish: &mut Vec<Vec<char>>) {
    loop {
        let mut swap_count = 0;

        for line in dish.iter_mut() {
            for idx in 0..(line.len() - 1) {
                if line[idx] == '.' && line[idx + 1] == 'O' {
                    line.swap(idx, idx + 1);
                    swap_count += 1;
                }
            }
        }

        match swap_count {
            0 => break,
            _ => continue,
        }
    }
}

fn move_rocks_south(dish: &mut Vec<Vec<char>>) {
    loop {
        let mut swap_count = 0;
        let mut dish_iterator = dish.iter_mut();
        let mut top_row = dish_iterator.next().unwrap();

        loop {
            match dish_iterator.next() {
                Some(vec) => {
                    let bottom_row = vec;

                    for (top_val, bottom_val) in top_row.iter_mut().zip(bottom_row.iter_mut()) {
                        if top_val == &'O' && bottom_val == &'.' {
                            *top_val = '.';
                            *bottom_val = 'O';
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

fn move_rocks_east(dish: &mut Vec<Vec<char>>) {
    loop {
        let mut swap_count = 0;

        for line in dish.iter_mut() {
            for idx in 0..(line.len() - 1) {
                if line[idx] == 'O' && line[idx + 1] == '.' {
                    line.swap(idx, idx + 1);
                    swap_count += 1;
                }
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
    fn test_move_rocks_north() {
        let mut line = vec![
            vec!['O', '.', '.'],
            vec!['.', 'O', '.'],
            vec!['.', '#', 'O'],
            vec!['#', 'O', 'O'],
            vec!['.', '.', 'O'],
        ];
        move_rocks_north(&mut line);
        assert_eq!(
            line,
            vec![
                vec!['O', 'O', 'O'],
                vec!['.', '.', 'O'],
                vec!['.', '#', 'O'],
                vec!['#', 'O', '.'],
                vec!['.', '.', '.']
            ]
        )
    }

    #[test]
    fn test_move_rocks_west() {
        let mut line = vec![
            vec!['O', '.', '.'],
            vec!['.', 'O', '.'],
            vec!['.', '.', 'O'],
            vec!['#', '.', 'O'],
            vec!['.', '#', 'O'],
        ];
        move_rocks_west(&mut line);
        assert_eq!(
            line,
            vec![
                vec!['O', '.', '.'],
                vec!['O', '.', '.'],
                vec!['O', '.', '.'],
                vec!['#', 'O', '.'],
                vec!['.', '#', 'O']
            ]
        )
    }

    #[test]
    fn test_move_rocks_south() {
        let mut line = vec![
            vec!['O', '.', 'O'],
            vec!['.', 'O', 'O'],
            vec!['.', '#', 'O'],
            vec!['#', 'O', '.'],
            vec!['.', '.', '.'],
        ];
        move_rocks_south(&mut line);
        assert_eq!(
            line,
            vec![
                vec!['.', '.', '.'],
                vec!['.', 'O', '.'],
                vec!['O', '#', 'O'],
                vec!['#', '.', 'O'],
                vec!['.', 'O', 'O']
            ]
        )
    }

    #[test]
    fn test_move_rocks_east() {
        let mut line = vec![
            vec!['O', '.', '.'],
            vec!['.', 'O', '.'],
            vec!['.', '.', 'O'],
            vec!['#', '.', 'O'],
            vec!['O', '#', '.'],
        ];
        move_rocks_east(&mut line);
        assert_eq!(
            line,
            vec![
                vec!['.', '.', 'O'],
                vec!['.', '.', 'O'],
                vec!['.', '.', 'O'],
                vec!['#', '.', 'O'],
                vec!['O', '#', '.']
            ]
        )
    }

    #[test]
    fn test_cycle_rocks() {
        let mut dish = vec![
            vec!['O', '.', '.', '.', '.', '#', '.', '.', '.', '.'],
            vec!['O', '.', 'O', 'O', '#', '.', '.', '.', '.', '#'],
            vec!['.', '.', '.', '.', '.', '#', '#', '.', '.', '.'],
            vec!['O', 'O', '.', '#', 'O', '.', '.', '.', '.', 'O'],
            vec!['.', 'O', '.', '.', '.', '.', '.', 'O', '#', '.'],
            vec!['O', '.', '#', '.', '.', 'O', '.', '#', '.', '#'],
            vec!['.', '.', '.', '.', '.', '.', '.', 'O', '.', '.'],
            vec!['#', '.', '.', '.', '.', '#', '#', '#', '.', '.'],
            vec!['#', 'O', 'O', '.', '.', '#', '.', '.', '.', '.'],
        ];

        cycle_rocks(&mut dish);
        assert_eq!(
            dish[0],
            vec!['.', '.', '.', '.', '.', '#', '.', '.', '.', '.']
        );

        cycle_rocks(&mut dish);
        assert_eq!(
            dish[0],
            vec!['.', '.', '.', '.', '.', '#', '.', '.', '.', '.']
        );

        cycle_rocks(&mut dish);
        assert_eq!(
            dish[0],
            vec!['.', '.', '.', '.', '.', '#', '.', '.', '.', '.']
        )
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
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

        assert_eq!(result, 64)
    }
}
