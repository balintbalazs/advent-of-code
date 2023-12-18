use std::fs;

fn hash(input: &str) -> usize {
    let mut val = 0;
    for ch in input.chars() {
        val += ch as usize;
        val *= 17;
        val &= 0xFF;
    }
    val
}

#[derive(Clone, Debug)]
struct Lens {
    label: String,
    focal_length: usize,
}

#[derive(Clone, Debug)]
struct Bx {
    lenses: Vec<Lens>,
}

impl Bx {
    fn new() -> Self {
        let lenses = Vec::new();
        Self { lenses }
    }

    fn find_index_by_label(&self, label: &str) -> Option<usize> {
        self.lenses.iter().position(|l| l.label == label)
    }

    fn remove(&mut self, label: &str) {
        if let Some(index) = self.find_index_by_label(label) {
            self.lenses.remove(index);
        }
    }

    fn insert(&mut self, lens: Lens) {
        if let Some(index) = self.find_index_by_label(&lens.label) {
            self.lenses[index].focal_length = lens.focal_length;
        } else {
            self.lenses.push(lens);
        }
    }
}

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("inputs/day15.txt").expect("Failed to read file");
    let sequence = input.trim().split(',');
    let part1: usize = sequence.clone().map(hash).sum();
    dbg!(part1);

    let mut boxes = vec![Bx::new(); 256];
    for step in sequence {
        if step.ends_with('-') {
            let label = &step[..step.len() - 1];
            let b = hash(label);
            boxes[b].remove(label);
        } else {
            let (label, focal_length) = step.split_once('=').unwrap();
            let b = hash(label);
            let lens = Lens {
                label: label.to_string(),
                focal_length: focal_length.parse().unwrap(),
            };
            boxes[b].insert(lens);
        }
    }
    // dbg!(&boxes);
    let part2: usize = boxes
        .iter()
        .enumerate()
        .map(|(bi, bx)| {
            bx.lenses
                .iter()
                .enumerate()
                .map(|(li, lens)| (bi + 1) * (li + 1) * lens.focal_length)
                .sum::<usize>()
        })
        .sum();
    dbg!(part2);
}
