use std::cmp::min;

struct Point {
    x: u32,
    y: u32,
    val: String,
}

impl Point {
    fn vector_in_aoe(&self, vector: &Vector) -> bool {
        if self.y == 0 && (self.y + 1) < vector.y {
            return false;
        }

        if (self.y - 1) > vector.y || (self.y + 1) < vector.y {
            return false;
        }

        if self.x == 0 && (self.x + 1) < vector.x_min {
            return false;
        }

        if (self.x - 1) > vector.x_max || (self.x + 1) < vector.x_min {
            return false;
        }

        true
    }
}

#[derive(Debug, Clone, Copy)]
struct Vector {
    x_min: u32,
    x_max: u32,
    y: u32,
    val: u32,
}

impl Vector {
    fn from_point(point: Point) -> Self {
        Self {
            x_min: point.x,
            x_max: point.x,
            y: point.y,
            val: point.val.parse::<u32>().unwrap(),
        }
    }

    fn add_point(&mut self, point: Point) {
        assert_eq!(self.y, point.y);

        if min(0, self.x_min.wrapping_sub(1)) == point.x {
            self.x_min = point.x;
            self.val = format!("{0}{1}", point.val, self.val)
                .parse::<u32>()
                .unwrap();
        } else if self.x_max + 1 == point.x {
            self.x_max = point.x;
            self.val = format!("{0}{1}", self.val, point.val)
                .parse::<u32>()
                .unwrap();
        }
    }

    fn valid_point(&self, point: &Point) -> bool {
        if self.y != point.y {
            return false;
        }

        if min(0, self.x_min.wrapping_sub(1)) != point.x {
            if self.x_max + 1 != point.x {
                return false;
            }
        }

        true
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let output = part_two(&input);
    dbg!(output);
}

fn part_two(input: &str) -> u32 {
    let gears = find_gears(input);
    let numbers = find_numbers(input);

    let mut gear_ratio_sum = 0;
    for gear in &gears {
        let mut adjacent_parts: Vec<Vector> = vec![];
        for number in &numbers {
            if gear.vector_in_aoe(number) {
                adjacent_parts.push(number.clone());
            }
        }

        if adjacent_parts.len() == 2 {
            let product = adjacent_parts[0].val * adjacent_parts[1].val;
            gear_ratio_sum += product;
        }

        adjacent_parts.clear();
    }

    gear_ratio_sum
}

fn find_gears(input: &str) -> Vec<Point> {
    let mut symbols = vec![];
    for (idx_y, line) in input.lines().enumerate() {
        for (idx_x, char) in line.chars().enumerate() {
            if char == '*' {
                let symbol = Point {
                    x: idx_x as u32,
                    y: idx_y as u32,
                    val: String::from(char),
                };
                symbols.push(symbol);
            }
        }
    }

    symbols
}

fn find_numbers(input: &str) -> Vec<Vector> {
    let mut numbers: Vec<Vector> = vec![];

    for (idx_y, line) in input.lines().enumerate() {
        for (idx_x, char) in line
            .chars()
            .enumerate()
            .filter(|(_, char)| char.is_ascii_digit())
        {
            let point = Point {
                x: idx_x as u32,
                y: idx_y as u32,
                val: String::from(char),
            };

            match numbers.last_mut() {
                Some(vec) => {
                    if vec.valid_point(&point) {
                        vec.add_point(point)
                    } else {
                        numbers.push(Vector::from_point(point))
                    }
                }
                None => numbers.push(Vector::from_point(point)),
            }
        }
    }

    numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two() {
        let result = part_two(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, 467835)
    }
}
