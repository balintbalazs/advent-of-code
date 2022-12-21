use std::{collections::HashMap, fs};

#[derive(Debug)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
struct Operation {
    lhs: String,
    rhs: String,
    operator: Operator,
}
#[derive(Debug)]
enum MonkeyJob {
    Shout(i64),
    Calculate(Operation),
}

#[derive(Debug)]
struct Monkeys(HashMap<String, MonkeyJob>);

impl Monkeys {
    fn from_str(input: &str) -> Self {
        let mut monkeys = HashMap::new();

        for line in input.lines() {
            let mut words = line.split(' ');
            let id = words.next().unwrap().trim_end_matches(':').to_string();
            let next = words.next().unwrap();
            let job = if let Ok(num) = next.parse::<i64>() {
                MonkeyJob::Shout(num)
            } else {
                let lhs = next.to_string();
                let operator = match words.next().unwrap() {
                    "+" => Operator::Add,
                    "-" => Operator::Sub,
                    "*" => Operator::Mul,
                    "/" => Operator::Div,
                    op => unreachable!("invalid operator {op}"),
                };
                let rhs = words.next().unwrap().to_string();
                MonkeyJob::Calculate(Operation { lhs, rhs, operator })
            };

            monkeys.insert(id, job);
        }

        Self(monkeys)
    }

    fn shout(&mut self, id: &str) -> i64 {
        let monkey_job = self.0.remove(id).unwrap();

        let num = match monkey_job {
            MonkeyJob::Shout(num) => num,
            MonkeyJob::Calculate(op) => {
                let lhs = self.shout(&op.lhs);
                let rhs = self.shout(&op.rhs);
                match op.operator {
                    Operator::Add => lhs + rhs,
                    Operator::Sub => lhs - rhs,
                    Operator::Mul => lhs * rhs,
                    Operator::Div => lhs / rhs,
                }
            }
        };

        let monkey_job = MonkeyJob::Shout(num);
        self.0.insert(id.to_string(), monkey_job);

        num
    }

    fn find_humn(&mut self, id: &str) -> i64 {
        let monkey_job = self.0.remove(id).unwrap();

        let num = match &monkey_job {
            MonkeyJob::Shout(num) => *num,
            MonkeyJob::Calculate(op) => {
                let lhs = self.find_humn(&op.lhs);
                let rhs = self.find_humn(&op.rhs);
                match op.operator {
                    Operator::Add => lhs + rhs,
                    Operator::Sub => lhs - rhs,
                    Operator::Mul => lhs * rhs,
                    Operator::Div => lhs / rhs,
                }
            }
        };

        // let new_job = MonkeyJob::Shout(num);
        self.0.insert(id.to_string(), monkey_job);

        todo!();
    }
}

fn main() {
    // Read the input from the file
    let input = fs::read_to_string("inputs/day21.txt").expect("Failed to read file");
    let mut monkeys = Monkeys::from_str(&input);
    dbg!(monkeys.shout("root"));
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_DATA: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn test_part1() {
        let mut monkeys = Monkeys::from_str(TEST_DATA);
        dbg!(monkeys.shout("root"));
    }
}
