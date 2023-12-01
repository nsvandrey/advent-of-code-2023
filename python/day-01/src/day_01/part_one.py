from typing import List


def read_input(fp: str) -> List[str]:
    with open(fp, "r") as file:
        return [line.strip() for line in file.readlines()]


def part_one(input: List[str]) -> int:
    output = 0
    for line in input:
        digits = [char for char in line if char.isnumeric()]
        calibration = int(digits[0] + digits[-1])
        output += calibration

    return output


def main():
    input = read_input("input.txt")
    output = part_one(input)
    print(output)


if __name__ == "__main__":
    main()
