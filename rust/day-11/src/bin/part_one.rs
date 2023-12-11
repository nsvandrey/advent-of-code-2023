use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let output = part_one(input);
    dbg!(output);
}

fn part_one(input: &str) -> usize {
    let universe: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut galaxies: Vec<[usize; 2]> = vec![];

    let mut empty_x: Vec<usize> = vec![];
    let mut empty_y: Vec<usize> = vec![];

    let mut seen_x: HashSet<usize> = HashSet::new();

    for (y, row) in universe.iter().enumerate() {
        if !row.iter().any(|char| *char == '#') {
            empty_y.push(y);
        } else {
            for (x, col) in row.iter().enumerate() {
                match col {
                    '#' => {
                        galaxies.push([x, y]);
                        seen_x.insert(x);
                    },
                    _ => continue,
                }
            }
        }
    }

    let x_len = universe[0].len();
    for idx in 0..x_len {
        if seen_x.contains(&idx) {
            continue
        } else {
            empty_x.push(idx);
        }
    }

    for galaxy in galaxies.iter_mut() {
        let x_offset = &empty_x.iter().filter(|elem| *elem < &galaxy[0]).count();
        let y_offset = &empty_y.iter().filter(|elem| *elem < &galaxy[1]).count();

        galaxy[0] = galaxy[0] + *x_offset;
        galaxy[1] = galaxy[1] + *y_offset;

    }

    let distance_sum: usize = (0..galaxies.len())
        .map(|idx| {
            let galaxy_subset = &galaxies[idx..];
            let source_galaxy = galaxy_subset[0];

            let mut total_distance = 0;

            for destination_galaxy in &galaxy_subset[1..] {
                let distance = euclidean_distance(&source_galaxy, destination_galaxy);
                total_distance += distance;
            }

            total_distance
        })
        .sum();

    distance_sum
}

fn euclidean_distance(point_a: &[usize; 2], point_b: &[usize; 2]) -> usize {
    let distance = (point_b[0] as isize - point_a[0] as isize).abs() + (point_b[1] as isize - point_a[1] as isize).abs();
    
    distance as usize
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        );
        assert_eq!(result, 374)
    }
}
