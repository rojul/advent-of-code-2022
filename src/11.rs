mod utils;

use std::{
    collections::VecDeque,
    fmt,
    str::{FromStr, Lines},
};

fn main() {
    let monkeys = parse(&utils::read_input(11));

    println!("{}", stuff_sling(&monkeys, 20, 3));
    println!("{}", stuff_sling(&monkeys, 10000, 1));
}

fn parse(input: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    let mut lines = input.lines();

    loop {
        monkeys.push(Monkey::parse(&mut lines));
        match lines.next() {
            None => return monkeys,
            Some("") => (),
            _ => panic!(),
        }
    }
}

fn stuff_sling(monkeys: &[Monkey], rounds: usize, relief_divisor: u64) -> usize {
    let mut inspected = vec![0; monkeys.len()];
    let mut items: Vec<VecDeque<_>> = monkeys.iter().map(|m| m.items.clone().into()).collect();

    let common_base = relief_divisor * monkeys.iter().map(|m| m.divisible_test).product::<u64>();

    for _ in 0..rounds {
        for (i, monkey) in monkeys.iter().enumerate() {
            while let Some(old_item) = items[i].pop_front() {
                inspected[i] += 1;
                let new_item =
                    (monkey.operation.op_assign(old_item) / relief_divisor) % common_base;
                let new_monkey = match new_item % monkey.divisible_test == 0 {
                    true => monkey.test_true_monkey,
                    false => monkey.test_false_monkey,
                };
                items[new_monkey].push_back(new_item);
            }
        }
    }

    inspected.sort();
    inspected.iter().rev().take(2).product()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    divisible_test: u64,
    test_true_monkey: usize,
    test_false_monkey: usize,
}

impl Monkey {
    fn parse(lines: &mut Lines) -> Self {
        lines.next().unwrap();

        Monkey {
            items: lines
                .next()
                .unwrap()
                .split_once(':')
                .unwrap()
                .1
                .split(',')
                .map(|item| item.trim().parse().unwrap())
                .collect(),
            operation: {
                let mut parts = lines.next().unwrap().rsplit(' ');
                let value = match parts.next().unwrap() {
                    "old" => None,
                    number => Some(number.parse().unwrap()),
                };
                match parts.next().unwrap() {
                    "+" => Operation::Add(value),
                    "*" => Operation::Mul(value),
                    _ => panic!(),
                }
            },
            divisible_test: parse_last(lines),
            test_true_monkey: parse_last(lines),
            test_false_monkey: parse_last(lines),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Add(Option<u64>),
    Mul(Option<u64>),
}

impl Operation {
    fn op_assign(self, old: u64) -> u64 {
        match self {
            Operation::Add(value) => old + value.unwrap_or(old),
            Operation::Mul(value) => old * value.unwrap_or(old),
        }
    }
}

fn parse_last<T: FromStr>(lines: &mut Lines) -> T
where
    T::Err: fmt::Debug,
{
    lines
        .next()
        .unwrap()
        .rsplit(' ')
        .next()
        .unwrap()
        .parse()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Monkey 0:
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
    fn test_monkey_parse() {
        assert_eq!(
            Monkey::parse(&mut EXAMPLE.lines()),
            Monkey {
                items: vec![79, 98],
                operation: Operation::Mul(Some(19)),
                divisible_test: 23,
                test_true_monkey: 2,
                test_false_monkey: 3,
            }
        )
    }

    #[test]
    fn example() {
        let monkeys = parse(EXAMPLE);
        assert_eq!(stuff_sling(&monkeys, 20, 3), 10605);
        assert_eq!(stuff_sling(&monkeys, 10000, 1), 2713310158);
    }
}
