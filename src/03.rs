mod utils;

fn main() {
    let input = utils::read_input(3);

    let part1: usize = input
        .lines()
        .map(|rucksack| rucksack.split_at(rucksack.len() / 2))
        .map(|compartments| find_error(compartments).unwrap())
        .map(item_priority)
        .sum();
    println!("{part1}");

    let part2: usize = input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|rucksacks| find_badge(rucksacks.try_into().unwrap()).unwrap())
        .map(item_priority)
        .sum();
    println!("{part2}");
}

fn find_error(compartments: (&str, &str)) -> Option<char> {
    compartments
        .0
        .chars()
        .find(|&item| compartments.1.contains(item))
}

fn find_badge(rucksacks: [&str; 3]) -> Option<char> {
    rucksacks[0]
        .chars()
        .find(|&item| rucksacks[1].contains(item) && rucksacks[2].contains(item))
}

fn item_priority(item: char) -> usize {
    let item = item as u8;
    let priority = if item >= b'a' {
        item - b'a' + 1
    } else {
        item - b'A' + 27
    };
    priority as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_error() {
        assert_eq!(find_error(("vJrwpWtwJgWr", "hcsFMMfFFhFp")), Some('p'));
    }

    #[test]
    fn test_find_badge() {
        assert_eq!(
            find_badge([
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg",
            ]),
            Some('r')
        );
    }

    #[test]
    fn test_item_priority() {
        assert_eq!(item_priority('a'), 1);
        assert_eq!(item_priority('z'), 26);
        assert_eq!(item_priority('A'), 27);
        assert_eq!(item_priority('Z'), 52);
    }
}
