use std::{fs, num::ParseIntError, str::FromStr};

#[derive(Debug, PartialEq)]
struct Range {
    source_start: u128,
    dest_start: u128,
    length: u128,
}

impl FromStr for Range {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = s.trim().split(' ');
        let dest_start: u128 = nums.next().unwrap().parse()?;
        let source_start: u128 = nums.next().unwrap().parse()?;
        let length: u128 = nums.next().unwrap().parse()?;

        Ok(Range {
            source_start,
            dest_start,
            length,
        })
    }
}

impl Range {
    fn apply(&self, source: u128) -> u128 {
        if (self.source_start..self.source_start + self.length).contains(&source) {
            source - self.source_start + self.dest_start
        } else {
            source
        }
    }

    fn contains(&self, num: u128) -> bool {
        self.source_start <= num && self.source_start + self.length > num
    }
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
}

impl Map {
    fn apply(&self, source: u128) -> u128 {
        for range in &self.ranges {
            if range.contains(source) {
                return range.apply(source);
            }
        }
        source
    }
}

impl FromStr for Map {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranges = s
            .lines()
            .map(|line| line.parse::<Range>().unwrap())
            .collect();
        Ok(Self { ranges })
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u128>,
    maps: Vec<Map>,
}

impl FromStr for Almanac {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blocks = s.split("\n\n");
        let (_, seeds) = blocks.next().unwrap().split_once(':').unwrap();
        let seeds = seeds
            .trim()
            .split(' ')
            .map(|n| n.parse::<u128>().unwrap())
            .collect();
        let maps = blocks
            .map(|block| {
                let (_, r) = block.split_once(':').unwrap();
                r.trim().parse().unwrap()
            })
            .collect();

        Ok(Self { seeds, maps })
    }
}

impl Almanac {
    fn part1(&self) -> u128 {
        self.seeds
            .iter()
            .map(|seed| {
                let mut id = *seed;
                for map in &self.maps {
                    id = map.apply(id);
                }
                id
            })
            .min()
            .unwrap()
    }

    fn part2(&self) -> u128 {
        self.seeds.chunks(2).map(|c| {
            match c {
                [start, length] => (*start..(*start + *length)).map(|seed| {
                    let mut id = seed;
                    for map in &self.maps {
                        id = map.apply(id);
                    }
                    id
                }).min().unwrap(),
                _ => panic!("can't deal with this"),
            }
        }).min().unwrap()
    }
}

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("inputs/day5.txt").expect("Failed to read file");
    let almanac: Almanac = input.parse().unwrap();
    dbg!(almanac.part1());
    dbg!(almanac.part2());
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_can_parse_line() {
        let line = "50 98 2";
        let range: Range = line.parse().unwrap();
        assert_eq!(
            range,
            Range {
                source_start: 98,
                dest_start: 50,
                length: 2
            }
        )
    }

    #[test]
    fn test_can_map() {
        let line = "50 98 2";
        let range: Range = line.parse().unwrap();
        let source = 99;
        let dest = 51;
        assert_eq!(range.apply(source), dest);
    }

    #[test]
    fn test_can_map_no_change() {
        let line = "50 98 2";
        let range: Range = line.parse().unwrap();
        let source = 100;
        let dest = 100;
        assert_eq!(range.apply(source), dest);
    }

    #[test]
    fn test_part1() {
        let input = {
            r#"seeds: 79 14 55 13

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
56 93 4"#
        };

        let a: Almanac = input.parse().unwrap();

        assert_eq!(a.seeds, vec![79, 14, 55, 13]);
        assert_eq!(a.maps.len(), 7);
        assert_eq!(
            a.maps[3].ranges[0],
            Range {
                source_start: 18,
                dest_start: 88,
                length: 7
            }
        );
        assert_eq!(a.part1(), 35);
    }

    #[test]
    fn test_part2() {
        let input = {
            r#"seeds: 79 14 55 13

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
56 93 4"#
        };

        let a: Almanac = input.parse().unwrap();
        assert_eq!(a.part2(), 46);
    }
}
