use std::collections::HashSet;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Beam {
    start_point: [usize; 2],
    direction: Direction,
}

fn main() {
    let input = include_str!("input.txt");
    let output = part_two(input);
    dbg!(output);
}

fn part_two(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut max_tiles = 0;

    for idx in 0..grid.len() {
        let start_point = [0, idx];
        let tiles = run_beam(&grid, start_point, Direction::Right);
        if tiles > max_tiles {
            max_tiles = tiles;
        }
    }

    for idx in 0..grid.len() {
        let start_point = [grid[0].len() - 1, idx];
        let tiles = run_beam(&grid, start_point, Direction::Left);
        if tiles > max_tiles {
            max_tiles = tiles;
        }
    }

    for idx in 0..grid[0].len() {
        let start_point = [idx, 0];
        let tiles = run_beam(&grid, start_point, Direction::Down);
        if tiles > max_tiles {
            max_tiles = tiles;
        }
    }

    for idx in 0..grid[0].len() {
        let start_point = [idx, grid.len() - 1];
        let tiles = run_beam(&grid, start_point, Direction::Up);
        if tiles > max_tiles {
            max_tiles = tiles;
        }
    }

    max_tiles
}

fn run_beam(grid: &Vec<Vec<char>>, start_point: [usize; 2], direction: Direction) -> usize {
    let mut energized: Vec<Vec<usize>> = vec![vec![0; grid[0].len()]; grid.len()];
    let initial_beam = Beam {
        start_point,
        direction,
    };
    let mut beams: Vec<Beam> = vec![initial_beam];
    let mut seen_beams: HashSet<Beam> = HashSet::new();

    while !beams.is_empty() {
        let current_beam = beams.pop().unwrap();
        match seen_beams.insert(current_beam.clone()) {
            true => track_beam(current_beam, &grid, &mut energized, &mut beams),
            false => continue,
        }
    }

    let result: usize = energized
        .iter()
        .map(|line| line.iter().sum::<usize>())
        .sum();

    result
}

fn track_beam(
    beam: Beam,
    grid: &Vec<Vec<char>>,
    energized: &mut Vec<Vec<usize>>,
    beams: &mut Vec<Beam>,
) {
    let mut current_point = beam.start_point;
    let mut current_direction = beam.direction;
    let mut seen_points: HashSet<Beam> = HashSet::new();

    while current_point[0] < grid[0].len() && current_point[1] < grid.len() {
        match seen_points.insert(Beam {
            start_point: current_point.clone(),
            direction: current_direction.clone(),
        }) {
            true => (),
            false => break,
        }

        energized[current_point[1]][current_point[0]] = 1;

        let current_char = grid[current_point[1]][current_point[0]];

        match current_char {
            '.' => match current_direction {
                Direction::Right => current_point = [current_point[0] + 1, current_point[1]],
                Direction::Left => {
                    current_point = [current_point[0].wrapping_sub(1), current_point[1]]
                }
                Direction::Up => {
                    current_point = [current_point[0], current_point[1].wrapping_sub(1)]
                }
                Direction::Down => current_point = [current_point[0], current_point[1] + 1],
            },
            '|' => match current_direction {
                Direction::Right | Direction::Left => {
                    beams.push(Beam {
                        start_point: current_point.clone(),
                        direction: Direction::Down,
                    });
                    current_point = [current_point[0], current_point[1].wrapping_sub(1)];
                    current_direction = Direction::Up;
                }
                Direction::Up => {
                    current_point = [current_point[0], current_point[1].wrapping_sub(1)]
                }
                Direction::Down => current_point = [current_point[0], current_point[1] + 1],
            },
            '-' => match current_direction {
                Direction::Right => current_point = [current_point[0] + 1, current_point[1]],
                Direction::Left => {
                    current_point = [current_point[0].wrapping_sub(1), current_point[1]]
                }
                Direction::Up | Direction::Down => {
                    beams.push(Beam {
                        start_point: current_point.clone(),
                        direction: Direction::Left,
                    });
                    current_point = [current_point[0] + 1, current_point[1]];
                    current_direction = Direction::Right;
                }
            },
            '/' => match current_direction {
                Direction::Right => {
                    current_point = [current_point[0], current_point[1].wrapping_sub(1)];
                    current_direction = Direction::Up;
                }
                Direction::Left => {
                    current_point = [current_point[0], current_point[1] + 1];
                    current_direction = Direction::Down;
                }
                Direction::Up => {
                    current_point = [current_point[0] + 1, current_point[1]];
                    current_direction = Direction::Right;
                }
                Direction::Down => {
                    current_point = [current_point[0].wrapping_sub(1), current_point[1]];
                    current_direction = Direction::Left;
                }
            },
            _ => match current_direction {
                Direction::Right => {
                    current_point = [current_point[0], current_point[1] + 1];
                    current_direction = Direction::Down;
                }
                Direction::Left => {
                    current_point = [current_point[0], current_point[1].wrapping_sub(1)];
                    current_direction = Direction::Up;
                }
                Direction::Up => {
                    current_point = [current_point[0].wrapping_sub(1), current_point[1]];
                    current_direction = Direction::Left;
                }
                Direction::Down => {
                    current_point = [current_point[0] + 1, current_point[1]];
                    current_direction = Direction::Right;
                }
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_two(
            r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....",
        );
        assert_eq!(result, 51)
    }
}
