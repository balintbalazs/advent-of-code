use std::{collections::HashMap, fs};
use num::integer::lcm;

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("inputs/day8.txt").expect("Failed to read file");
    let mut lines = input.lines();
    let instructions = lines.next().unwrap();
    let nodes = lines
        .skip(1)
        .map(|line| {
            let id = line[0..3].to_string();
            let left = line[7..10].to_string();
            let right = line[12..15].to_string();
            (id, (left, right))
        })
        .collect::<HashMap<_, _>>();

    // part1
    let mut current = "AAA".to_string();

    for (i, instr) in instructions.chars().cycle().enumerate() {
        let path = nodes.get(&current).unwrap();
        // dbg!(&node);
        current = match instr {
            'L' => path.0.clone(),
            'R' => path.1.clone(),
            _ => panic!("invalid instruction"),
        };
        if &current == "ZZZ" {
            let part1 = i + 1;
            dbg!(part1);
            break;
        }
    }

    //part2
    let starts: Vec<_> = nodes
        .keys()
        .filter(|id| id.ends_with('A'))
        .cloned()
        .collect();

    let periods: Vec<_> = starts.into_iter().map(|start| {

        let mut current = start.clone();
        let mut first_z = None;
        let mut offset = 0;
        let mut period = 0;
        for (i, instr) in instructions.chars().cycle().enumerate() {
            let path = nodes.get(&current).unwrap();
            // dbg!(&node);
            current = match instr {
                'L' => path.0.clone(),
                'R' => path.1.clone(),
                _ => panic!("invalid instruction"),
            };
            if current.ends_with('Z') {
                // dbg!(&current, i);
                if let Some(first_z) = first_z {
                    // dbg!(&first_z);
                    // dbg!((current, i));
                    period = i - offset;
                    break;
                }
                else {
                    first_z = Some((current.clone(), i));
                    offset = i;
                }
            }
        }
        // dbg!(offset);
        // dbg!(period);
        period
    }).collect();

    let part2 = periods.into_iter().reduce(lcm).unwrap();
    dbg!(part2);
}
