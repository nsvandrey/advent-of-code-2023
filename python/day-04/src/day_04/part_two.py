from typing import List


def read_input(fp: str) -> List[str]:
    with open(fp, "r") as file:
        input = [line.strip() for line in file.readlines()]
        return input


def part_two(input: List[str]) -> int:
    scratchcards = [1] * len(input)

    for idx, line in enumerate(input):
        matching_numbers = parse_line(line)
        scratchcard_copies = scratchcards[idx]

        lower_bound = idx + 1
        upperbound = min(lower_bound + matching_numbers, len(input))
        new_cards = scratchcards[lower_bound:upperbound]

        scratchcards[lower_bound:upperbound] = list(map(lambda x: x + scratchcard_copies, new_cards))

    return sum(scratchcards)


def parse_line(line: str) -> int:
    winning_numbers, scratchcard_numbers = line.split(":")[1].split("|")
    winning_numbers = set([int(number.strip()) for number in winning_numbers.split()])
    scratchcard_numbers = set([int(number.strip()) for number in scratchcard_numbers.split()])

    return len(winning_numbers.intersection(scratchcard_numbers))


def main():
    input = read_input("input.txt")
    output = part_two(input)
    print(output)


if __name__ == "__main__":
    main()
