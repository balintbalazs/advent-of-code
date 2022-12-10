use std::fs;

const ON: &str = "#";
const OFF: &str = " ";

const WIDTH: usize = 40;
const HEIGHT: usize = 6;

#[derive(Debug)]
struct Cpu {
    regx: i32,
    clock: i32,
    interesting_clocks: Vec<i32>,
    sum_interesting_signal: i32,
    crt: Vec<&'static str>,
}

impl Cpu {
    fn new(interesting_clocks: &[i32]) -> Self {
        Self {
            regx: 1,
            clock: 0,
            interesting_clocks: interesting_clocks.into(),
            sum_interesting_signal: 0,
            crt: vec![OFF; WIDTH * HEIGHT],
        }
    }

    fn clock(&mut self) {
        if self.regx - 1 <= self.clock % WIDTH as i32 && self.regx + 1 >= self.clock % WIDTH as i32
        {
            self.crt[self.clock as usize] = ON;
        }
        self.clock += 1;
        if self.interesting_clocks.contains(&self.clock) {
            self.sum_interesting_signal += self.regx * self.clock;
        }
    }

    fn noop(&mut self) {
        self.clock();
    }

    fn addx(&mut self, op: i32) {
        self.clock();
        self.clock();
        self.regx += op;
    }

    fn execute(&mut self, commands: &str) {
        for line in commands.lines() {
            let mut line = line.split(" ");
            let cmd = line.next().unwrap();
            match cmd {
                "noop" => self.noop(),
                "addx" => self.addx(line.next().unwrap().parse().unwrap()),
                _ => unreachable!(),
            }
        }
    }

    fn print_crt(&self) {
        for (i, c) in self.crt.iter().enumerate() {
            if i % WIDTH == 0 {
                println!();
            }
            print!("{c}");
        }
        println!();
    }
}

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("inputs/day10.txt").expect("Failed to read file");
    let interesting = [20, 60, 100, 140, 180, 220];
    let mut cpu = Cpu::new(&interesting);
    cpu.execute(&input);
    dbg!(cpu.sum_interesting_signal);
    cpu.print_crt();
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = {
        r###"noop
addx 3
addx -5
"###
    };

    const LONGER_TEST_DATA: &str = {
        r###"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
"###
    };

    #[test]
    fn can_execute_short() {
        let interesting = [];
        let mut cpu = Cpu::new(&interesting);
        cpu.execute(&TEST_DATA);
        assert_eq!(-1, cpu.regx);
    }

    #[test]
    fn can_executu_long() {
        let interesting = [20, 60, 100, 140, 180, 220];
        let mut cpu = Cpu::new(&interesting);
        cpu.execute(&LONGER_TEST_DATA);
        assert_eq!(13140, cpu.sum_interesting_signal);
    }

    #[test]
    fn can_draw() {
        let interesting = [20, 60, 100, 140, 180, 220];
        let mut cpu = Cpu::new(&interesting);
        cpu.execute(&LONGER_TEST_DATA);
        cpu.print_crt();
    }
}
