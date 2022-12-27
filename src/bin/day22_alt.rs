use std::{collections::HashSet, fs};

#[derive(Clone, Copy, Debug)]
#[repr(i8)]
enum Facing {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Facing {
    fn turn(&mut self, t: Turn) {
        let f = *self as i8 + t as i8;
        let f = f.rem_euclid(4);
        *self = match f {
            0 => Facing::Right,
            1 => Facing::Down,
            2 => Facing::Left,
            3 => Facing::Up,
            _ => unimplemented!(),
        };
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Pos {
    r: usize,
    c: usize,
}

#[derive(Debug, Clone, Copy)]
#[repr(i8)]
enum Turn {
    Left = -1,
    Right = 1,
}
#[derive(Debug)]
enum Move {
    Forward(usize),
    Turn(Turn),
}

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    data: Vec<Vec<char>>,
    obstacles: HashSet<Pos>,
    starting_col: usize,
}

impl Map {
    fn from_str(input: &str) -> Self {
        let lines: Vec<_> = input.lines().collect();
        let height = lines.len();
        let width = lines.iter().map(|l| l.len()).max().unwrap();
        let starting_col = lines[0].chars().position(|ch| ch == '.').unwrap();

        let mut data = vec![vec![' '; width]; height];
        let mut obstacles = HashSet::new();

        for (r, &line) in lines.iter().enumerate() {
            for (c, ch) in line.chars().enumerate() {
                data[r][c] = ch;
                // obstacles
                if ch == '#' {
                    obstacles.insert(Pos { r, c });
                }
            }
        }

        Self {
            width,
            height,
            data,
            obstacles,
            starting_col,
        }
    }

    fn starting_col(&self) -> usize {
        self.starting_col
    }
}

#[derive(Debug)]
struct Board {
    map: Map,
    facing: Facing,
    position: Pos,
    moves: Vec<Move>,
}

impl Board {
    fn from_str(input: &str) -> Self {
        let input = input.to_string().replace("\r\n", "\n");
        let mut parts = input.split("\n\n");
        let map_input = parts.next().unwrap();
        let map = Map::from_str(map_input);

        let position = Pos {
            r: 0,
            c: map.starting_col(),
        };

        let moves_input = parts.next().unwrap();
        let mut moves = vec![];
        let mut buffer = "".to_string();

        for c in moves_input.chars() {
            match c {
                '0'..='9' => buffer.push(c),
                'L' => {
                    if buffer.len() > 0 {
                        let num = buffer.parse().unwrap();
                        buffer = "".to_string();
                        moves.push(Move::Forward(num));
                    }
                    moves.push(Move::Turn(Turn::Left));
                }
                'R' => {
                    if buffer.len() > 0 {
                        let num = buffer.parse().unwrap();
                        buffer = "".to_string();
                        moves.push(Move::Forward(num));
                    }
                    moves.push(Move::Turn(Turn::Right));
                }
                c => panic!("Invalid char in moves {c}"),
            }
        }
        if buffer.len() > 0 {
            let num = buffer.parse().unwrap();
            moves.push(Move::Forward(num));
        }

        Self {
            map,
            facing: Facing::Right,
            position,
            moves,
        }
    }

    fn do_moves(&mut self) {
        for m in &self.moves {
            match m {
                Move::Turn(t) => self.facing.turn(*t),
                Move::Forward(n) => {
                    for _ in 0..*n {
                        let mut r = self.position.r;
                        let mut c = self.position.c;
                        match self.facing {
                            Facing::Down => {
                                r += 1;
                                if r == self.map.height || self.map.data[r][c] == ' ' {
                                    r -= 1;
                                    while r > 0 && self.map.data[r][c] != ' ' {
                                        r -= 1;
                                    }
                                }
                            }
                            Facing::Right => {
                                c += 1;
                                if c == self.map.width || self.map.data[r][c] == ' ' {
                                    c -= 1;
                                    while c > 0 && self.map.data[r][c] != ' ' {
                                        c -= 1;
                                    }
                                }
                            }
                            Facing::Up => {
                                if r == 0 || self.map.data[r - 1][c] == ' ' {
                                    while r < self.map.height - 1 && self.map.data[r][c] != ' ' {
                                        r += 1;
                                    }
                                } else {
                                    r -= 1;
                                }
                            }
                            Facing::Left => {
                                if c == 0 || self.map.data[r][c - 1] == ' ' {
                                    while c < self.map.width - 1 && self.map.data[r][c] != ' ' {
                                        c += 1;
                                    }
                                } else {
                                    c -= 1;
                                }
                            }
                        }
                        let new_pos = Pos { r, c };
                        if self.map.obstacles.contains(&new_pos) {
                            break;
                        } else {
                            self.position = new_pos;
                        }
                    }
                }
            }
        }
    }

    fn password(&self) -> usize {
        (self.position.r + 1) * 1000 + (self.position.c + 1) * 4 + self.facing as usize
    }
}

fn main() {
    let input = fs::read_to_string("inputs/day22.txt").expect("Failed to read file");
    let mut board = Board::from_str(&input);
    board.do_moves();
    dbg!(board.password());
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_DATA: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn test_part1() {
        let mut board = Board::from_str(TEST_DATA);
        board.do_moves();
        dbg!(&board.position);
        dbg!(&board.facing);
        dbg!(board.password());
    }
}
