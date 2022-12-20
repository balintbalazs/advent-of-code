use std::{collections::VecDeque, fs};

fn from_str(input: &str) -> VecDeque<(usize, i64)> {
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .enumerate()
        .collect()
}

fn find_index_by_id(list: &VecDeque<(usize, i64)>, id: usize) -> usize {
    list.iter().position(|(i, _)| *i == id).unwrap()
}

fn find_index_by_value(list: &VecDeque<(usize, i64)>, value: i64) -> usize {
    list.iter().position(|(_, v)| *v == value).unwrap()
}

fn move_element(list: &mut VecDeque<(usize, i64)>, id: usize) {
    let index = find_index_by_id(&list, id);
    let item = list.remove(index).unwrap();
    let len = list.len();
    let new_index = (index as i64 + item.1).rem_euclid(len as i64) as usize;
    list.insert(new_index, item);
}

fn print_items(list: &VecDeque<(usize, i64)>) {
    for item in list {
        print!("{}, ", item.1)
    }
    println!();
    println!();
}

fn calculate_coordinates(list: &VecDeque<(usize, i64)>) -> i64 {
    let zero_index = find_index_by_value(list, 0);
    let i1 = (zero_index + 1000) % list.len();
    let i2 = (zero_index + 2000) % list.len();
    let i3 = (zero_index + 3000) % list.len();

    list[i1].1 + list[i2].1 + list[i3].1
}

fn part1(input: &str) -> i64 {
    let mut list = from_str(input);
    for id in 0..list.len() {
        move_element(&mut list, id);
        // print_items(&list);
    }
    calculate_coordinates(&list)
}

const DECRYPTION_KEY: i64 = 811589153;

fn part2(input: &str) -> i64 {
    let mut list = from_str(input);

    list = list
        .into_iter()
        .map(|(id, value)| (id, value * DECRYPTION_KEY))
        .collect();

    for _ in 0..10 {
        for id in 0..list.len() {
            move_element(&mut list, id);
        }
    }
    calculate_coordinates(&list)
}

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("inputs/day20.txt").expect("Failed to read file");
    dbg!(part1(&input));
    dbg!(part2(&input));
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_DATA: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn test_part1() {
        assert_eq!(3, part1(TEST_DATA));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1623178306, part2(TEST_DATA));
    }
}
