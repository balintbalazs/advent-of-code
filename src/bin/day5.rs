use std::fs;

use scan_fmt::scan_fmt;

fn main() {
    let input_stacks = fs::read_to_string("inputs/day5_stacks.txt").expect("Failed to read file");
    let mut orig_stacks = vec![];
    for line in input_stacks.lines().rev() {
        for (index, crat) in line.chars().skip(1).step_by(4).enumerate() {
            match crat {
                ' ' => (),
                _ => {
                    if orig_stacks.len() <= index {
                        orig_stacks.push(vec![crat]);
                    } else {
                        orig_stacks[index].push(crat);
                    }
                }
            };
        }
    }

    let input_moves = fs::read_to_string("inputs/day5_moves.txt").expect("Failed to read file");

    let mut stacks = orig_stacks.clone();
    for line in input_moves.lines() {
        let (amount, from, to) =
            scan_fmt!(line, "move {d} from {d} to {d}", usize, usize, usize).unwrap();
        for _ in 0..amount {
            let crat = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(crat);
        }
    }
    let top_crates: String = stacks.iter().map(|stack| stack.last().unwrap()).collect();
    dbg!(top_crates);

    let mut stacks = orig_stacks.clone();
    for line in input_moves.lines() {
        let (amount, from, to) =
            scan_fmt!(line, "move {d} from {d} to {d}", usize, usize, usize).unwrap();
        let mut crane = vec![];
        for _ in 0..amount {
            let crat = stacks[from - 1].pop().unwrap();
            crane.push(crat);
        }
        for crat in crane.into_iter().rev() {
            stacks[to - 1].push(crat);
        }
    }

    let top_crates_part2: String = stacks.iter().map(|stack| stack.last().unwrap()).collect();
    dbg!(top_crates_part2);
}
