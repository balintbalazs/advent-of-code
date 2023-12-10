use std::{collections::VecDeque, fs};

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use Direction::{Down, Left, Right, Up};

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("inputs/day10.txt").expect("Failed to read file");
    let lines = input.lines();
    let mut start = (0, 0);
    let mut tiles = lines
        .enumerate()
        .map(|(row, line)| {
            let ch = line
                .chars()
                .enumerate()
                .map(|(col, ch)| {
                    if ch == 'S' {
                        start = (row, col);
                    }
                    ch
                })
                .collect::<Vec<_>>();
            ch
        })
        .collect::<Vec<_>>();

    let mut starts = Vec::new();
    let mut dirs = Vec::new();

    let up = (start.0 - 1, start.1);
    if let '7' | '|' | 'F' = tiles[up.0][up.1] {
        starts.push(up);
        dirs.push(Up);
    }

    let left = (start.0, start.1 - 1);
    if let 'F' | '-' | 'L' = tiles[left.0][left.1] {
        starts.push(left);
        dirs.push(Left);
    }

    let right = (start.0, start.1 + 1);
    if let '7' | '-' | 'J' = tiles[right.0][right.1] {
        starts.push(right);
        dirs.push(Right);
    }

    let down = (start.0 + 1, start.1);
    if let 'J' | '|' | 'L' = tiles[down.0][down.1] {
        starts.push(down);
        dirs.push(Down);
    }

    let starting_dirs = [dirs[0], dirs[1]];

    assert_eq!(starts.len(), 2);

    let mut part1 = 1;

    while starts[0] != starts[1] {
        for (i, (d, l)) in dirs.iter_mut().zip(starts.iter_mut()).enumerate() {
            *d = match d {
                Up => match tiles[l.0][l.1] {
                    'F' => Right,
                    '7' => Left,
                    '|' => Up,
                    _ => panic!("invalid direction"),
                },
                Down => match tiles[l.0][l.1] {
                    'L' => Right,
                    'J' => Left,
                    '|' => Down,
                    _ => panic!("invalid direction"),
                },
                Right => match tiles[l.0][l.1] {
                    '7' => Down,
                    'J' => Up,
                    '-' => Right,
                    _ => panic!("invalid direction"),
                },
                Left => match tiles[l.0][l.1] {
                    'F' => Down,
                    'L' => Up,
                    '-' => Left,
                    _ => panic!("invalid direction"),
                },
            };
            match d {
                Up => l.0 -= 1,
                Down => l.0 += 1,
                Left => l.1 -= 1,
                Right => l.1 += 1,
            }
        }

        part1 += 1;
    }

    dbg!(part1);

    // replace starting tile with matching pipe
    tiles[start.0][start.1] = match starting_dirs {
        [Up, Down] | [Down, Up] => '|',
        [Left, Right] | [Right, Left] => '-',
        [Left, Up] | [Up, Left] => 'J',
        [Right, Up] | [Up, Right] => 'L',
        [Left, Down] | [Down, Left] => '7',
        [Right, Down] | [Down, Right] => 'F',
        _ => panic!("invalid starting directions"),
    };

    // create bigger maze, replace every tile by 3x3 sub-tiles
    // '.' is for empty space
    // 'x' is for wall
    let width = tiles[0].len();
    let height = tiles.len();

    let mut maze = vec![vec![' '; width * 3]; height * 3];

    for r in 0..height {
        for c in 0..width {
            let template = match tiles[r][c] {
                'F' => [['.', '.', '.'], ['.', 'x', 'x'], ['.', 'x', '.']],
                'J' => [['.', 'x', '.'], ['x', 'x', '.'], ['.', '.', '.']],
                '7' => [['.', '.', '.'], ['x', 'x', '.'], ['.', 'x', '.']],
                'L' => [['.', 'x', '.'], ['.', 'x', 'x'], ['.', '.', '.']],
                '|' => [['.', 'x', '.'], ['.', 'x', '.'], ['.', 'x', '.']],
                '-' => [['.', '.', '.'], ['x', 'x', 'x'], ['.', '.', '.']],
                '.' => [['.', '.', '.'], ['.', '.', '.'], ['.', '.', '.']],
                _ => panic!("invalid tile"),
            };
            for rr in 0..3 {
                for cc in 0..3 {
                    maze[3 * r + rr][3 * c + cc] = template[rr][cc];
                }
            }
        }
    }

    // now in upscaled maze to flood fill from (0,0)
    // label outside tiles with 'o'
    // assume (0,0) is outside
    let mut q = VecDeque::new();
    q.push_back((0, 0));
    while let Some((r, c)) = q.pop_front() {
        if maze[r][c] == '.' {
            maze[r][c] = 'o'; //outside
            if r > 0 {
                q.push_back((r - 1, c));
            }
            if c > 0 {
                q.push_back((r, c - 1));
            }
            if r < height * 3 - 1 {
                q.push_back((r + 1, c));
            }
            if c < width * 3 - 1 {
                q.push_back((r, c + 1));
            }
        }
    }

    // count tiles on the inside
    // this assumes there is only a single loop
    // tile is on the inside if all 9 sub-tiles are empty '.' or walls 'x'
    let mut part2 = 0;

    for r in 0..height {
        'tiles: for c in 0..width {
            let mut count = 0;
            for rr in 0..3 {
                for cc in 0..3 {
                    let m = maze[3 * r + rr][3 * c + cc];
                    if m == '.' || m == 'x' {
                        count += 1;
                    }
                }
            }
            if count == 9 {
                part2 += 1;
            }
        }
    }

    dbg!(part2);
}
