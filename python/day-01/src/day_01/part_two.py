from typing import Dict, List, Tuple

DIGIT_MAP: Dict[str, str] = {
    "one": "1", "1": "1",
    "two": "2", "2": "2",
    "three": "3", "3": "3",
    "four": "4", "4": "4",
    "five": "5", "5": "5",
    "six": "6", "6": "6",
    "seven": "7", "7": "7",
    "eight": "8", "8": "8",
    "nine": "9", "9": "9",
}

def read_input(fp: str) -> List[str]:
    with open(fp, "r") as file:
        return [line.strip() for line in file.readlines()]
    

def part_two(input: List[str]) -> int:
    output = 0
    for line in input:
        digit_idxs = []
        for digit in DIGIT_MAP.keys():
            digit_idxs += parse_line(line, digit)
    
        digit_idxs = sorted(digit_idxs, key=lambda x: x[0])
        calibration = int(digit_idxs[0][1] + digit_idxs[-1][1])
        output += calibration
    
    return output

def parse_line(line: str, digit: str) -> List[Tuple[int, int]]:
    digit_idxs = []
    idx = 0
    while True:
        try:
            idx = line.index(digit, idx)
            digit_idxs.append((idx, DIGIT_MAP[digit]))
            idx += 1
        except ValueError:
            break

    return digit_idxs

def main():
    input = read_input("input.txt")
    output = part_two(input)
    print(output)


if __name__ == "__main__":
    main()