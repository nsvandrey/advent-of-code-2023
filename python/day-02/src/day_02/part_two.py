from typing import Dict, List


def read_input(fp: str) -> List[str]:
    with open(fp, "r") as file:
        return [line.strip() for line in file.readlines()]


def part_two(input: List[str]) -> int:
    output = 0
    for line in input:
        output += parse_line(line)

    return output


def parse_line(line: str) -> int:
    max_red, max_green, max_blue = 0, 0, 0
    records = line.split(": ")[1]

    for record in records.split("; "):
        cubes = parse_game_record(record)
        if cubes.get("red", 0) > max_red:
            max_red = cubes.get("red")
        if cubes.get("green", 0) > max_green:
            max_green = cubes.get("green")
        if cubes.get("blue", 0) > max_blue:
            max_blue = cubes.get("blue")

    return max_red * max_green * max_blue


def parse_game_record(record: str) -> Dict[str, int]:
    cubes = record.split(", ")
    return {cube.split()[1]: int(cube.split()[0]) for cube in cubes}


def main():
    input = read_input("input.txt")
    output = part_two(input)
    print(output)


if __name__ == "__main__":
    main()
