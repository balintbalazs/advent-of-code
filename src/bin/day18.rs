use std::{
    collections::{HashSet, VecDeque},
    fs,
};

#[derive(PartialEq, Eq)]
enum Part {
    Part1,
    Part2,
}

use crate::Part::*;

fn calculate_surface(input: &str, part: Part) -> usize {
    let cubes: Vec<_> = input
        .lines()
        .map(|line| {
            let mut nums = line.split(',').map(|num| num.parse::<i32>().unwrap());
            (
                nums.next().unwrap(),
                nums.next().unwrap(),
                nums.next().unwrap(),
            )
        })
        .collect();
    let mut total_surface = cubes.len() * 6;

    for i in 0..cubes.len() {
        for j in i..cubes.len() {
            if adjacent(cubes[i], cubes[j]) {
                total_surface -= 2;
            }
        }
    }

    if part == Part1 {
        return total_surface;
    }
    let cubes: HashSet<_> = cubes.into_iter().collect();

    let max_x = cubes.iter().map(|(x, _, _)| x).max().unwrap() + 1;
    let max_y = cubes.iter().map(|(_, y, _)| y).max().unwrap() + 1;
    let max_z = cubes.iter().map(|(_, _, z)| z).max().unwrap() + 1;

    let min_x = cubes.iter().map(|(x, _, _)| x).min().unwrap() - 1;
    let min_y = cubes.iter().map(|(_, y, _)| y).min().unwrap() - 1;
    let min_z = cubes.iter().map(|(_, _, z)| z).min().unwrap() - 1;

    let min = (min_x, min_y, min_z);

    let mut q = VecDeque::new();
    let mut visited = HashSet::new();
    let directions = vec![
        (1, 0, 0),
        (0, 1, 0),
        (0, 0, 1),
        (-1, 0, 0),
        (0, -1, 0),
        (0, 0, -1),
    ];
    q.push_back(min);

    let mut outside_surface = 0;

    while let Some(cube) = q.pop_front() {
        if visited.contains(&cube) {
            continue;
        }
        visited.insert(cube);
        for (dx, dy, dz) in directions.iter() {
            let x = cube.0 + dx;
            let y = cube.1 + dy;
            let z = cube.2 + dz;
            if x >= min_x && x <= max_x && y >= min_y && y <= max_y && z >= min_z && z <= max_z {
                if cubes.contains(&(x, y, z)) {
                    outside_surface += 1;
                } else {
                    q.push_back((x, y, z));
                }
            }
        }
    }

    outside_surface
}

fn adjacent(i: (i32, i32, i32), j: (i32, i32, i32)) -> bool {
    (i.0 - j.0).abs() + (i.1 - j.1).abs() + (i.2 - j.2).abs() == 1
}

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("inputs/day18.txt").expect("Failed to read file");
    dbg!(calculate_surface(&input, Part1));
    dbg!(calculate_surface(&input, Part2));
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_DATA: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn test_part1() {
        assert_eq!(64, calculate_surface(TEST_DATA, Part1));
    }
    #[test]
    fn test_part2() {
        assert_eq!(58, calculate_surface(TEST_DATA, Part2));
    }
}
