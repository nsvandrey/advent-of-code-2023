import unittest

from day_01.part_one import part_one

test_data = """1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"""

class TestDayOnePartOne(unittest.TestCase):
    def setUp(self) -> None:
        self.data = [line.strip() for line in test_data.split("\n")]
    
    def test_part_one(self):
        self.assertEqual(part_one(self.data), 142)

if __name__ == "__main__":
    unittest.main()