import unittest

from day_03.part_one import part_one

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


class TestDayThreePartOne(unittest.TestCase):
    def setUp(self) -> None:
        self.data = [line.strip() for line in test_data.split("\n")]

    def test_part_one(self):
        self.assertEqual(part_one(self.data), 4361)


if __name__ == "__main__":
    unittest.main()
