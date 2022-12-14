mod utils;

use std::{cmp::Ordering, iter::Peekable, str::Chars};

fn main() {
    let pairs = parse(&utils::read_input(13));

    println!("{}", part1(&pairs));
    println!("{}", part2(&pairs));
}

fn parse(input: &str) -> Vec<(Vec<Value>, Vec<Value>)> {
    let mut pairs = Vec::new();
    let mut lines = input.lines();

    loop {
        pairs.push((
            parse_packet(lines.next().unwrap()),
            parse_packet(lines.next().unwrap()),
        ));
        match lines.next() {
            None => return pairs,
            Some("") => (),
            _ => panic!(),
        }
    }
}

fn parse_packet(line: &str) -> Vec<Value> {
    match parse_value(&mut line.chars().peekable()) {
        Value::Array(array) => array,
        _ => panic!(),
    }
}

fn parse_value(chars: &mut Peekable<Chars>) -> Value {
    match chars.next() {
        Some(c) if c.is_ascii_digit() => {
            let mut number = c.to_string();
            while let Some(c) = chars.next_if(|c| c.is_ascii_digit()) {
                number.push(c);
            }
            Value::Number(number.parse().unwrap())
        }
        Some('[') => {
            let mut array = Vec::new();
            while chars.next_if_eq(&']').is_none() {
                array.push(parse_value(chars));
                chars.next_if_eq(&',');
            }
            Value::Array(array)
        }
        _ => panic!(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    Number(u32),
    Array(Vec<Value>),
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::Number(left), Value::Number(right)) => left.cmp(right),
            (Value::Array(left), Value::Array(right)) => left
                .iter()
                .zip(right)
                .map(|(left, right)| left.cmp(right))
                .find(|&ordering| ordering != Ordering::Equal)
                .unwrap_or_else(|| left.len().cmp(&right.len())),
            (Value::Number(_), Value::Array(_)) => Value::Array(vec![self.clone()]).cmp(other),
            (Value::Array(_), Value::Number(_)) => self.cmp(&Value::Array(vec![other.clone()])),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(pairs: &[(Vec<Value>, Vec<Value>)]) -> usize {
    pairs
        .iter()
        .enumerate()
        .filter(|(_, (a, b))| a < b)
        .map(|(i, _)| i + 1)
        .sum()
}

fn part2(pairs: &[(Vec<Value>, Vec<Value>)]) -> usize {
    let dividers = [&vec![Value::Number(2)], &vec![Value::Number(6)]];

    let mut packets: Vec<_> = pairs.iter().flat_map(|(a, b)| [a, b]).collect();
    packets.extend(dividers);
    packets.sort();

    dividers
        .into_iter()
        .map(|divider| {
            packets
                .iter()
                .position(|&packet| packet == divider)
                .unwrap()
                + 1
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_parse_packet() {
        use Value::*;

        assert_eq!(
            parse_packet("[[],[1,[2]]]"),
            [
                Array(vec![]),
                Array(vec![Number(1), Array(vec![Number(2)])])
            ]
        )
    }

    #[test]
    fn test_compare() {
        for (left, right, order) in [
            ("[1,1,3,1,1]", "[1,1,5,1,1]", true),
            ("[[1],[2,3,4]]", "[[1],4]", true),
            ("[9]", "[[8,7,6]]", false),
            ("[[4,4],4,4]", "[[4,4],4,4,4]", true),
            ("[7,7,7,7]", "[7,7,7]", false),
            ("[]", "[3]", true),
            ("[[[]]]", "[[]]", false),
            (
                "[1,[2,[3,[4,[5,6,7]]]],8,9]",
                "[1,[2,[3,[4,[5,6,0]]]],8,9]",
                false,
            ),
        ] {
            assert_eq!(
                parse_packet(left) < parse_packet(right),
                order,
                "{left} < {right}"
            );
        }
    }

    #[test]
    fn example() {
        let pairs = parse(EXAMPLE);
        assert_eq!(part1(&pairs), 13);
        assert_eq!(part2(&pairs), 140);
    }
}
