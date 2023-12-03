from dataclasses import dataclass
from typing import List, Tuple


@dataclass
class Point:
    x: int
    y: int
    val: str

    def vector_in_aoe(self, vec: "Vector") -> bool:
        if (self.y - 1) > vec.y or (self.y + 1) < vec.y:
            return False

        if (self.x - 1) > vec.x_max or (self.x + 1) < vec.x_min:
            return False

        return True


@dataclass
class Vector:
    x_min: int
    x_max: int
    y: int
    _val: str

    @classmethod
    def from_point(cls, point: Point):
        return cls(x_min=point.x, x_max=point.x, y=point.y, _val=point.val)

    def add_point(self, point: Point):
        assert point.y == self.y

        if point.x == self.x_min - 1:
            self.x_min = point.x
            self._val = point.val + self._val
        elif point.x == self.x_max + 1:
            self.x_max = point.x
            self._val += point.val

    @property
    def val(self):
        return int(self._val)


def read_input(fp: str) -> List[List[str]]:
    with open(fp, "r") as file:
        input = [line.strip() for line in file.readlines()]

    return input


def part_two(input: List[str]) -> int:
    gears, numbers = find_gears_and_numbers(input)

    gear_ratio_sum = 0
    for gear in gears:
        adjacent_parts = []
        for number in numbers:
            if gear.vector_in_aoe(number):
                adjacent_parts.append(number)

        if len(adjacent_parts) == 2:
            gear_ratio_sum += adjacent_parts[0].val * adjacent_parts[1].val

    return gear_ratio_sum


def find_gears_and_numbers(
    input: List[List[str]],
) -> Tuple[List[Point], List[Vector]]:
    gear_coordinates = []
    number_vectors = []

    for idx_y, line in enumerate(input):
        current_vector = None
        for idx_x, col in enumerate(line):
            if not col.isnumeric():
                if current_vector:
                    number_vectors.append(current_vector)
                    current_vector = None

                if col == "*":
                    point = Point(idx_x, idx_y, col)
                    gear_coordinates.append(point)

            if col.isnumeric():
                point = Point(idx_x, idx_y, col)
                if not current_vector:
                    current_vector = Vector.from_point(point)
                else:
                    current_vector.add_point(point)

        if current_vector:
            number_vectors.append(current_vector)
            current_vector = None

    return (gear_coordinates, number_vectors)


def main():
    input = read_input("input.txt")
    output = part_two(input)
    print(output)


if __name__ == "__main__":
    main()
