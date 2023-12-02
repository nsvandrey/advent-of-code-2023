from typing import Dict, List, Optional

MAX_RED = 12
MAX_GREEN = 13
MAX_BLUE = 14


def read_input(fp: str) -> List[str]:
    with open(fp, "r") as file:
        return [line.strip() for line in file.readlines()]


def part_one(input: List[str]) -> int:
    output = 0
    for line in input:
        output += parse_line(line)

    return output


def parse_line(line: str) -> int:
    game_id, records = line.split(": ")

    for record in records.split("; "):
        cubes = parse_game_record(record)
        if (
            cubes.get("red", 0) > MAX_RED
            or cubes.get("green", 0) > MAX_GREEN
            or cubes.get("blue", 0) > MAX_BLUE
        ):
            return 0

    game_id = game_id.split()[1]

    return int(game_id)


def parse_game_record(record: str) -> Dict[str, int]:
    cubes = record.split(", ")
    return {cube.split()[1]: int(cube.split()[0]) for cube in cubes}


def main():
    input = read_input("input.txt")
    output = part_one(input)
    print(output)


if __name__ == "__main__":
    main()
