import unittest

from day_01.part_two import part_two

test_data = """two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"""

class TestDayOnePartTwo(unittest.TestCase):
    def setUp(self) -> None:
        self.data = [line.strip() for line in test_data.split("\n")]
    
    def test_part_two(self):
        self.assertEqual(part_two(self.data), 281)

if __name__ == "__main__":
    unittest.main()