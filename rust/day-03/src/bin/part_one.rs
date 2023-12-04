use std::cmp::min;

#[derive(Debug)]
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

#[derive(Debug)]
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
    let output = part_one(&input);
    dbg!(output);
}

fn part_one(input: &str) -> u32 {
    let symbols = find_symbols(input);
    let numbers = find_numbers(input);

    let mut engine_schematic_sum = 0;
    for number in &numbers {
        for symbol in &symbols {
            if symbol.vector_in_aoe(number) {
                engine_schematic_sum += number.val;
                break;
            }
        }
    }

    engine_schematic_sum
}

fn find_symbols(input: &str) -> Vec<Point> {
    let mut symbols = vec![];
    for (idx_y, line) in input.lines().enumerate() {
        for (idx_x, char) in line.chars().enumerate() {
            if !char.is_ascii_digit() && char != '.' {
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
    fn test_part_one() {
        let result = part_one(
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
        assert_eq!(result, 4361)
    }
}
