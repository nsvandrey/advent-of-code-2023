import unittest

from day_03.part_two import part_two

test_data = """467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."""


class TestDayThreePartTwo(unittest.TestCase):
    def setUp(self) -> None:
        self.data = [line.strip() for line in test_data.split("\n")]

    def test_part_two(self):
        self.assertEqual(part_two(self.data), 467835)


if __name__ == "__main__":
    unittest.main()
