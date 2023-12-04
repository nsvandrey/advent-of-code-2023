from typing import List


def read_input(fp: str) -> List[str]:
    with open(fp, "r") as file:
        input = [line.strip() for line in file.readlines()]
        return input


def part_one(input: List[str]) -> int:
    points = 0
    for line in input:
        winning_count = parse_line(line)
        if winning_count == 0:
            continue
        else:
            points += 2 ** (winning_count - 1)

    return points


def parse_line(line: str) -> int:
    winning_numbers, scratchcard_numbers = line.split(":")[1].split("|")
    winning_numbers = set([int(number.strip()) for number in winning_numbers.split()])
    scratchcard_numbers = set([int(number.strip()) for number in scratchcard_numbers.split()])

    return len(winning_numbers.intersection(scratchcard_numbers))


def main():
    input = read_input("input.txt")
    output = part_one(input)
    print(output)


if __name__ == "__main__":
    main()
