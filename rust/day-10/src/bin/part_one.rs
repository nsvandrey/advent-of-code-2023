fn main() {
    let input = include_str!("input.txt");
    let output = part_one(input);
    dbg!(output);
}

fn part_one(input: &str) -> usize {
    let pipe_grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let start_point = find_start_point(&pipe_grid);
    let start_connection = find_start_connection(&pipe_grid, &start_point);

    let mut prev_point = start_point.clone();
    let mut current_point = start_connection.clone();

    let mut steps = 1;

    while current_point != start_point {
        let next_point = find_next_point(&pipe_grid, &current_point, &prev_point);
        prev_point = current_point;
        current_point = next_point;

        steps += 1;
    }

    steps / 2
}

fn find_start_point(grid: &Vec<Vec<char>>) -> [usize; 2] {
    let mut start_point = None;

    for (y, row) in grid.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col == 'S' {
                start_point = Some([x, y]);
                break;
            }
        }
    }

    start_point.unwrap()
}

fn find_start_connection(grid: &Vec<Vec<char>>, start_point: &[usize; 2]) -> [usize; 2] {
    let north = grid[start_point[1] - 1][start_point[0]];
    let west = grid[start_point[1]][start_point[0] + 1];

    match north {
        '|' | '7' | 'F' => return [start_point[0], start_point[1] - 1],
        _ => match west {
            '-' | 'J' | '7' => return [start_point[0] + 1, start_point[1]],
            _ => return [start_point[0], start_point[1] + 1],
        },
    }
}

fn find_next_point(
    grid: &Vec<Vec<char>>,
    current_point: &[usize; 2],
    prev_point: &[usize; 2],
) -> [usize; 2] {
    let current_char = grid[current_point[1]][current_point[0]];

    match current_char {
        '|' => {
            if current_point[1] > prev_point[1] {
                return [current_point[0], current_point[1] + 1];
            } else {
                return [current_point[0], current_point[1] - 1];
            }
        }
        '-' => {
            if current_point[0] > prev_point[0] {
                return [current_point[0] + 1, current_point[1]];
            } else {
                return [current_point[0] - 1, current_point[1]];
            }
        }
        'L' => {
            if current_point[0] < prev_point[0] {
                return [current_point[0], current_point[1] - 1];
            } else {
                return [current_point[0] + 1, current_point[1]];
            }
        }
        'J' => {
            if current_point[1] > prev_point[1] {
                return [current_point[0] - 1, current_point[1]];
            } else {
                return [current_point[0], current_point[1] - 1];
            }
        }
        '7' => {
            if current_point[0] > prev_point[0] {
                return [current_point[0], current_point[1] + 1];
            } else {
                return [current_point[0] - 1, current_point[1]];
            }
        }
        'F' => {
            if current_point[0] < prev_point[0] {
                return [current_point[0], current_point[1] + 1];
            } else {
                return [current_point[0] + 1, current_point[1]];
            }
        }
        _ => panic!("Here there be dragons!"),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one_a() {
        let result = part_one(
            ".....
.S-7.
.|.|.
.L-J.
.....",
        );
        assert_eq!(result, 4)
    }

    #[test]
    fn test_part_one_b() {
        let result = part_one(
            "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
        );
        assert_eq!(result, 8)
    }
}
