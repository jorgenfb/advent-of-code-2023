fn main() {
    let input = include_str!("input.txt");
    print!("Result: {}\n", process(input));
}

#[derive(Debug)]
struct Item {
    dest_start: u64,
    source_start: u64,
    range_length: u64,
}

struct LookupTable {
    items: Vec<Item>,
}

impl LookupTable {
    fn from(mut items: Vec<Item>) -> LookupTable {
        items.sort_by_key(|item| item.source_start);
        LookupTable { items: items }
    }

    fn lookup(&self, source: &u64) -> u64 {
        let item_id = match self
            .items
            .binary_search_by_key(&source, |item| &item.source_start)
        {
            Ok(i) => i, // If found, use the index
            Err(i) => {
                if i == 0 {
                    return *source; // If the index is 0, we're at the start, not finding a lookup, exit early
                }
                i - 1 // Look at the item before the index where the value would be placed
            }
        };

        let item = &self.items[item_id];
        if item.source_start + item.range_length > *source {
            return item.dest_start + (source - item.source_start);
        }

        *source
    }
}

fn parse_map<'a>(input: &mut impl Iterator<Item = &'a str>) -> LookupTable {
    LookupTable::from(
        input
            .skip(1)
            .take_while(|line| !line.is_empty()) // Take until empty line
            .map(|line| {
                let mut parts = line.split(" ").map(|s| s.parse::<u64>().unwrap());
                let destination_start = parts.next().unwrap();
                let source_start = parts.next().unwrap();
                let range_length = parts.next().unwrap();

                Item {
                    dest_start: destination_start,
                    source_start: source_start,
                    range_length: range_length,
                }
            })
            .collect(),
    )
}

fn process(input: &str) -> u64 {
    let mut lines = input.lines();

    // parse seeds
    let values = lines.next().unwrap()[7..]
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let seed_ranges = values
        .chunks_exact(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect::<Vec<_>>();

    lines.next();

    let seed_to_soil = parse_map(&mut lines);
    let soil_to_fertilizer = parse_map(&mut lines);
    let fertilizer_to_water = parse_map(&mut lines);
    let water_to_light = parse_map(&mut lines);
    let light_to_temperature = parse_map(&mut lines);
    let temperature_to_humidity = parse_map(&mut lines);
    let humidity_to_location = parse_map(&mut lines);

    println!("Number of seed ranges: {}", seed_ranges.len());

    let mut min = std::u64::MAX;
    for (seed_start, seed_len) in seed_ranges {
        let seed_end = seed_start + seed_len;
        for seed in seed_start..seed_end {
            let soil = seed_to_soil.lookup(&seed);
            let fertilizer = soil_to_fertilizer.lookup(&soil);
            let water = fertilizer_to_water.lookup(&fertilizer);
            let light = water_to_light.lookup(&water);
            let temperature = light_to_temperature.lookup(&light);
            let humidity = temperature_to_humidity.lookup(&temperature);
            let location = humidity_to_location.lookup(&humidity);
            if location < min {
                min = location;
            }
        }
    }
    min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "seeds: 79 14 55 13

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
56 93 4";

        assert_eq!(process(input), 46);
    }
}
