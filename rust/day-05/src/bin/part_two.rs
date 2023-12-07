use std::cmp::min;

struct Map {
    source_start: u64,
    source_end: u64,
    dest_start: u64,
}

impl Map {
    fn from_map_str(map_str: &str) -> Self {
        let map_elems = map_str
            .split_whitespace()
            .map(|elem| elem.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        Self {
            source_start: map_elems[1],
            source_end: map_elems[1] + map_elems[2],
            dest_start: map_elems[0],
        }
    }

    fn val(&self, int: &u64) -> Option<u64> {
        if *int < self.source_start || *int > self.source_end {
            None
        } else {
            Some(self.dest_start + (int - self.source_start))
        }
    }
}
fn main() {
    let input = include_str!("input.txt");
    let output = part_two(input);
    dbg!(output);
}

fn part_two(input: &str) -> u64 {
    let input_iterator = input.split("\n\n");
    let seeds = input_iterator
        .clone()
        .take(1)
        .next()
        .unwrap()
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .split_whitespace()
        .map(|elem| elem.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut map_iterator = input_iterator
        .skip(1)
        .map(|map_range_str| parse_map(map_range_str));

    let seed_to_soil_map = map_iterator.next().unwrap();
    let soil_to_fertilizer_map = map_iterator.next().unwrap();
    let fetrilizer_to_water_map = map_iterator.next().unwrap();
    let water_to_light_map = map_iterator.next().unwrap();
    let light_to_temperature_map = map_iterator.next().unwrap();
    let temperature_to_humidity_map = map_iterator.next().unwrap();
    let humidity_to_location_map = map_iterator.next().unwrap();

    let min_loc = seeds
        .chunks(2)
        .map(|chunk| (chunk[0]..=chunk[0] + chunk[1]).collect::<Vec<u64>>())
        .map(|seed_chunk| {
            find_seed_location(
                seed_chunk,
                &seed_to_soil_map,
                &soil_to_fertilizer_map,
                &fetrilizer_to_water_map,
                &water_to_light_map,
                &light_to_temperature_map,
                &temperature_to_humidity_map,
                &humidity_to_location_map,
            )
        })
        .min()
        .unwrap();

    min_loc
}

fn find_seed_location(
    seeds: Vec<u64>,
    seed_to_soil_map: &Vec<Map>,
    soil_to_fertilizer_map: &Vec<Map>,
    fetrilizer_to_water_map: &Vec<Map>,
    water_to_light_map: &Vec<Map>,
    light_to_temperature_map: &Vec<Map>,
    temperature_to_humidity_map: &Vec<Map>,
    humidity_to_location_map: &Vec<Map>,
) -> u64 {
    let mut min_loc = u64::MAX;

    for seed in seeds {
        let soil = process(&seed, seed_to_soil_map);
        let fertilizer = process(&soil, &soil_to_fertilizer_map);
        let water = process(&fertilizer, &fetrilizer_to_water_map);
        let light = process(&water, &water_to_light_map);
        let temperature = process(&light, &light_to_temperature_map);
        let humidity = process(&temperature, &temperature_to_humidity_map);
        let location = process(&humidity, &humidity_to_location_map);

        min_loc = min(min_loc, location);
    }

    min_loc
}

fn parse_map(map_str: &str) -> Vec<Map> {
    let map_iterator = map_str.lines().skip(1);
    let map_vec = map_iterator
        .map(|map_str| Map::from_map_str(map_str))
        .collect::<Vec<Map>>();

    map_vec
}

fn process(seed: &u64, maps: &Vec<Map>) -> u64 {
    for map in maps {
        if let Some(num) = map.val(seed) {
            return num;
        }
    }

    *seed
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_two() {
        let result = part_two(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        );
        assert_eq!(result, 46)
    }
}
