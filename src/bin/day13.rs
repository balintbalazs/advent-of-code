use serde::Deserialize;
use std::{cmp::Ordering, fs};

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(untagged)]
enum List {
    N(u32),
    L(Vec<List>),
}

use crate::List::*;

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (N(a), N(b)) => a.cmp(b),
            (N(a), L(_)) => L(vec![N(*a)]).cmp(other),
            (L(_), N(b)) => self.cmp(&L(vec![N(*b)])),
            (L(a), L(b)) => match (a.len(), b.len()) {
                (0, 0) => Ordering::Equal,
                (0, _) => Ordering::Less,
                (_, 0) => Ordering::Greater,
                (_, _) => match a[0].cmp(&b[0]) {
                    Ordering::Equal => L(a[1..].to_vec()).cmp(&L(b[1..].to_vec())),
                    ord => ord,
                },
            },
        }
    }
}

fn part1(input: &str) -> u32 {
    input
        .replace("\r\n", "\n")
        .split("\n\n")
        .map(|pair| {
            let pair: Vec<_> = pair.split("\n").collect();
            let l0: List = serde_json::from_str(pair[0]).unwrap();
            let l1: List = serde_json::from_str(pair[1]).unwrap();
            (l0, l1)
        })
        .zip(1..)
        .filter(|(pair, _)| pair.0 < pair.1)
        .map(|(_, index)| index)
        .sum()
}

fn part2(input: &str) -> u32 {
    let mut packets: Vec<List> = input
        .replace("\r\n", "\n")
        .replace("\n\n", "\n")
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();

    let divider1 = L(vec![L(vec![N(2)])]);
    let divider2 = L(vec![L(vec![N(6)])]);

    packets.push(divider1.clone());
    packets.push(divider2.clone());

    packets.sort();

    let mut decoder_key = 1;
    for (packet, index) in packets.iter().zip(1..) {
        if packet == &divider1 || packet == &divider2 {
            decoder_key *= index;
        }
    }
    decoder_key
}

fn main() {
    let input = fs::read_to_string("inputs/day13.txt").expect("Failed to read file");

    dbg!(part1(&input));

    dbg!(part2(&input));
}

#[cfg(test)]
mod tests {

    const TEST_DATA: &str = {
        "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"
    };

    use super::*;

    #[test]
    fn can_parse_list() {
        let list = TEST_DATA.lines().nth(3).unwrap();
        let list: List = serde_json::from_str(list).unwrap();

        dbg!(list);
    }

    #[test]
    fn can_compare() {
        let a = N(3);
        let b = L(vec![N(2)]);
        assert_eq!(Some(Ordering::Greater), a.partial_cmp(&b));
    }

    #[test]
    fn part1_test() {
        assert_eq!(13, part1(TEST_DATA));
    }

    #[test]
    fn part2_test() {
        assert_eq!(140, part2(TEST_DATA));
    }
}
