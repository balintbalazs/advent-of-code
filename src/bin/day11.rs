use color_eyre::{
    eyre::{eyre, Context},
    Result,
};
use std::{fs, num::ParseIntError};

use crate::Part::{Part1, Part2};

#[derive(PartialEq)]
enum Part {
    Part1,
    Part2,
}

#[derive(Debug)]
enum Operation {
    Add,
    Mul,
}

impl Operation {
    fn new(s: &str) -> Result<Self> {
        match s {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Mul),
            op => Err(eyre!("Invalid operation {op}")),
        }
    }
}

#[derive(Debug)]
enum Operand {
    Old,
    Num(u64),
}

impl Operand {
    fn new(s: &str) -> Result<Self> {
        Ok(match s {
            "old" => Self::Old,
            s => Self::Num(s.parse()?),
        })
    }
}

#[derive(Debug)]
struct Function {
    lhs: Operand,
    rhs: Operand,
    operation: Operation,
}

impl Function {
    fn new(line: &str) -> Result<Self> {
        let line = line
            .trim()
            .strip_prefix("Operation: new = ")
            .ok_or_else(|| eyre!("Operation missing"))?;
        let mut tokens = line.split(' ');
        let lhs = tokens
            .next()
            .ok_or_else(|| eyre!("lhs missing for operation"))?;
        let operation = tokens
            .next()
            .ok_or_else(|| eyre!("lhs missing for operation"))?;
        let rhs = tokens
            .next()
            .ok_or_else(|| eyre!("lhs missing for operation"))?;
        Ok(Self {
            lhs: Operand::new(lhs)?,
            rhs: Operand::new(rhs)?,
            operation: Operation::new(operation)?,
        })
    }

    fn call(&self, item: u64) -> u64 {
        // dbg!(&self);
        // dbg!(&item);
        let lhs = match self.lhs {
            Operand::Old => item,
            Operand::Num(num) => num,
        };
        let rhs = match self.rhs {
            Operand::Old => item,
            Operand::Num(num) => num,
        };
        match self.operation {
            Operation::Add => lhs + rhs,
            Operation::Mul => lhs * rhs,
        }
    }
}

fn get_last_num_in_line(line: &str) -> Result<usize> {
    let num = line
        .split(' ')
        .last()
        .ok_or_else(|| eyre!("line is empty"))?;
    Ok(num.parse()?)
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    inspect_function: Function,
    test_num: u64,
    true_target: usize,
    false_target: usize,
    num_inspections: u64,
}

impl Monkey {
    fn new(input: &str) -> Result<Self> {
        let mut lines = input.lines();
        lines.next(); // skip monkey number - they are ascending

        let items = lines
            .next()
            .ok_or_else(|| eyre!("Missing starting items line"))?
            .split(": ")
            .nth(1)
            .ok_or_else(|| eyre!("Missing starting items numbers"))?
            .split(", ")
            .map(|num| num.parse().map_err(|e: ParseIntError| eyre!(e)))
            .collect::<Result<Vec<u64>>>()?;

        let next_line = lines
            .next()
            .ok_or_else(|| eyre!("Missing operation for monkey"))?;
        let inspect_function = Function::new(next_line)?;

        let next_line = lines
            .next()
            .ok_or_else(|| eyre!("Missing test for monkey"))?;
        let test_num = get_last_num_in_line(next_line)?;

        let next_line = lines
            .next()
            .ok_or_else(|| eyre!("Missing true target for monkey"))?;
        let true_target = get_last_num_in_line(next_line)?;

        let next_line = lines
            .next()
            .ok_or_else(|| eyre!("Missing false target for monkey"))?;
        let false_target = get_last_num_in_line(next_line)?;

        Ok(Self {
            items,
            inspect_function,
            test_num: test_num as u64,
            true_target,
            false_target,
            num_inspections: 0,
        })
    }

    fn inspect_and_throw(&mut self, part: &Part) -> Vec<(usize, u64)> {
        let res = self
            .items
            .iter()
            .map(|item| {
                let mut worry_level = self.inspect_function.call(*item);
                if part == &Part1 {
                    worry_level /= 3;
                }
                self.num_inspections += 1;
                let target_monkey = if worry_level % self.test_num == 0 {
                    self.true_target
                } else {
                    self.false_target
                };
                (target_monkey, worry_level)
            })
            .collect();
        self.items = vec![]; //monkey doesn't throw to itself
        res
    }

    fn receive_item(&mut self, item: u64) {
        self.items.push(item);
    }
}

#[derive(Debug)]
struct Monkeys {
    monkeys: Vec<Monkey>,
    test_num_multiples: u64,
}

impl Monkeys {
    fn new(input: &str) -> Result<Self> {
        let input = input.replace("\r\n", "\n");
        let monkeys = input.split("\n\n");
        let maybe_monkeys: Result<Vec<_>> = monkeys
            .map(Monkey::new)
            .enumerate()
            .map(|(i, res)| res.with_context(|| format!("failed to parse monkey {i}")))
            .collect();
        let monkeys = maybe_monkeys?;
        let test_num_multiples = monkeys.iter().map(|m| m.test_num).product::<u64>();
        Ok(Self {
            monkeys,
            test_num_multiples,
        })
    }

    fn round(&mut self, part: Part) -> Result<()> {
        for m in 0..self.monkeys.len() {
            let throws = self.monkeys[m].inspect_and_throw(&part);
            for (target_monkey, item) in throws {
                if target_monkey >= self.monkeys.len() {
                    return Err(eyre!(
                        "Monkey {m} tried to throw to a non-existent monkey ({target_monkey})"
                    ));
                }
                self.monkeys[target_monkey].receive_item(item % self.test_num_multiples);
            }
        }
        Ok(())
    }

    fn top_bois(&self) -> Vec<u64> {
        let mut inspections: Vec<u64> = self.monkeys.iter().map(|m| m.num_inspections).collect();
        inspections.sort_by(|a, b| b.cmp(a));
        inspections
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let input = fs::read_to_string("inputs/day11.txt").wrap_err("reading inputs/day11.txt")?;

    let mut monkeys = Monkeys::new(&input)?;

    for _ in 0..20 {
        monkeys.round(Part1)?;
    }

    let part1: u64 = monkeys.top_bois().iter().take(2).product();
    dbg!(part1);

    let mut monkeys = Monkeys::new(&input)?;

    for _ in 0..10000 {
        monkeys.round(Part2)?;
    }

    let part2: u64 = monkeys.top_bois().iter().take(2).product();
    dbg!(part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_round_part1() -> Result<()> {
        let mut monkeys = Monkeys::new(TEST_INPUT)?;

        for _ in 0..20 {
            monkeys.round(Part1)?;
            // dbg!((i,&monkeys));
        }
        assert_eq!(101, monkeys.monkeys[0].num_inspections);

        // assert_eq!(vec![10, 12, 14, 26, 34], monkeys.monkeys[0].items);
        // assert_eq!(vec![245, 93, 53, 199, 115], monkeys.monkeys[1].items);

        Ok(())
    }

    #[test]
    fn test_round_part2() -> Result<()> {
        let mut monkeys = Monkeys::new(TEST_INPUT)?;

        for _ in 0..20 {
            monkeys.round(Part2)?;
        }
        assert_eq!(99, monkeys.monkeys[0].num_inspections);

        for _ in 20..1000 {
            monkeys.round(Part2)?;
        }
        assert_eq!(5204, monkeys.monkeys[0].num_inspections);

        for _ in 1000..10000 {
            monkeys.round(Part2)?;
        }
        assert_eq!(52166, monkeys.monkeys[0].num_inspections);

        Ok(())
    }
}
