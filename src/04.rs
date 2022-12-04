mod utils;

use std::ops::RangeInclusive;

fn main() {
    let assignments: Vec<_> = utils::read_input(4)
        .lines()
        .map(|line| {
            split_once_map(line, ',', |s| {
                let range = split_once_map(s, '-', |s| s.parse::<u32>().unwrap());
                range.0..=range.1
            })
        })
        .collect();

    let part1 = assignments
        .iter()
        .filter(|(a, b)| a.contains_range(b) || b.contains_range(a))
        .count();
    println!("{part1}");

    let part2 = assignments
        .iter()
        .filter(|(a, b)| a.intersects(b) || b.intersects(a))
        .count();
    println!("{part2}");
}

fn split_once_map<R, F: Fn(&str) -> R>(s: &str, c: char, f: F) -> (R, R) {
    let parts = s.split_once(c).unwrap();
    (f(parts.0), f(parts.1))
}

trait RangeExt {
    fn contains_range(&self, other: &Self) -> bool;
    fn intersects(&self, other: &Self) -> bool;
}

impl RangeExt for RangeInclusive<u32> {
    fn contains_range(&self, other: &Self) -> bool {
        self.start() <= other.start() && self.end() >= other.end()
    }

    fn intersects(&self, other: &Self) -> bool {
        self.contains(other.start()) || self.contains(other.end())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_range() {
        assert_eq!((1..=3).contains_range(&(4..=6)), false);
        assert_eq!((4..=6).contains_range(&(1..=3)), false);
        assert_eq!((1..=5).contains_range(&(2..=4)), true);
        assert_eq!((1..=3).contains_range(&(1..=3)), true);
    }

    #[test]
    fn test_intersects() {
        assert_eq!((1..=3).intersects(&(4..=6)), false);
        assert_eq!((4..=6).intersects(&(1..=3)), false);
        assert_eq!((1..=5).intersects(&(2..=4)), true);
        assert_eq!((1..=3).intersects(&(3..=5)), true);
        assert_eq!((1..=5).intersects(&(3..=7)), true);
    }
}
