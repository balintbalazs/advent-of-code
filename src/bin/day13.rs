use std::fs;

#[derive(Debug, PartialEq)]
enum SymmetryLine {
    Horizontal(usize),
    Vertical(usize),
}

fn find_symmetry(
    pattern: &Vec<Vec<char>>,
    orig_symm: Option<&SymmetryLine>,
) -> Option<SymmetryLine> {
    let hsymm = match orig_symm {
        Some(SymmetryLine::Horizontal(n)) => Some(*n),
        _ => None,
    };
    let vsymm = match orig_symm {
        Some(SymmetryLine::Vertical(n)) => Some(*n),
        _ => None,
    };
    if let Some(n) = find_horizontal_symmetry(pattern, hsymm) {
        Some(SymmetryLine::Horizontal(n))
    } else {
        let pattern = transpose_pattern(pattern);
        find_horizontal_symmetry(&pattern, vsymm).map(SymmetryLine::Vertical)
    }
}

fn find_horizontal_symmetry(
    pattern: &Vec<Vec<char>>,
    orig_symm: Option<usize>,
) -> Option<usize> {
    'outer: for r in 0..(pattern.len() - 1) {
        // dbg!(r);
        let mut rt = r + 1;
        let mut rb = r + 1;
        while rt > 0 && rb < pattern.len() {
            // dbg!(rt);
            // dbg!(rb);
            if pattern[rt - 1] != pattern[rb] {
                continue 'outer;
            }
            rt -= 1;
            rb += 1;
        }
        if orig_symm != Some(r + 1) {
            return Some(r + 1);
        }
    }
    None
}

fn transpose_pattern(pattern: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let h = pattern.len();
    let w = pattern[0].len();
    let mut t = vec![vec![' '; h]; w];

    for r in 0..w {
        for c in 0..h {
            t[r][c] = pattern[c][r];
        }
    }
    t
}

fn swap(ch: &char) -> char {
    if *ch == '.' {
        '#'
    } else {
        '.'
    }
}

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("inputs/day13.txt").expect("Failed to read file");
    let patterns = input.split("\n\n");

    let part1 = patterns
        .clone()
        .map(|pattern| {
            let pattern = pattern.lines().map(|line| line.chars().collect()).collect();
            let s = find_symmetry(&pattern, None).unwrap();
            // dbg!(&s);
            match s {
                SymmetryLine::Vertical(n) => n,
                SymmetryLine::Horizontal(n) => 100 * n,
            }
        })
        .sum::<usize>();
    dbg!(part1);

    let part2 = patterns
        .clone()
        .map(|pattern| {
            let pattern = pattern
                .lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>();
            // let h = pattern.len();
            // let w = pattern[0].len();
            let orig_symm = find_symmetry(&pattern, None).unwrap();
            for (r, line) in pattern.iter().enumerate() {
                for (c, ch) in line.iter().enumerate() {
                    let mut p = pattern.clone();
                    p[r][c] = swap(ch);
                    // for line in p.iter() {
                    //     for ch in line.iter() {
                    //         print!("{ch}");
                    //     }
                    //     println!();
                    // }
                    let s = find_symmetry(&p, Some(&orig_symm));
                    // dbg!(&s);
                    if let Some(s) = s {
                        // dbg!((r, c));
                        if s != orig_symm {
                            // dbg!(&s);
                            return match s {
                                SymmetryLine::Vertical(n) => n,
                                SymmetryLine::Horizontal(n) => 100 * n,
                            };
                        }
                    }
                    // println!();
                }
            }
            panic!("Shouldn't reach here");
            // match orig_symm {
            //     SymmetryLine::Vertical(n) => n,
            //     SymmetryLine::Horizontal(n) => 100 * n,
            // }
        })
        .sum::<usize>();
    dbg!(part2);
}
