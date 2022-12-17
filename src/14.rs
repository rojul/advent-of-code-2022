mod utils;

fn main() {
    let parsed = parse(&utils::read_input(14));

    println!("{}", part1(&parsed));
    println!("{}", part2(&parsed));
}

const SIZE: usize = 1000;
type Cave = Vec<[bool; SIZE]>;

fn parse(data: &str) -> Cave {
    let mut cave = Vec::new();
    for line in data.lines() {
        let mut coords = line.split(" -> ").map(|s| {
            let (a, b) = s.split_once(',').unwrap();
            (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap())
        });
        let mut prev = coords.next().unwrap();
        for coord in coords {
            for y in prev.1.min(coord.1)..=prev.1.max(coord.1) {
                while cave.len() <= y {
                    cave.push([false; SIZE]);
                }
                for x in prev.0.min(coord.0)..=prev.0.max(coord.0) {
                    cave[y][x] = true;
                }
            }
            prev = coord;
        }
    }
    cave
}

fn next_unit(cave: &Cave) -> Option<(usize, usize)> {
    let mut x = 500;
    let mut y = 0;
    'y: loop {
        let next_y = y + 1;
        let Some(row) = cave.get(next_y) else {
            break;
        };
        for next_x in [x, x - 1, x + 1] {
            if !row[next_x] {
                x = next_x;
                y = next_y;
                continue 'y;
            }
        }
        break;
    }
    (!cave[y][x]).then_some((x, y))
}

fn part1(cave: &Cave) -> usize {
    let mut cave = cave.clone();
    let mut count = 0;
    loop {
        let (x, y) = next_unit(&cave).unwrap();
        if y >= cave.len() - 1 {
            break;
        }
        cave[y][x] = true;
        count += 1;
    }
    count
}

fn part2(cave: &Cave) -> usize {
    let mut cave = cave.clone();
    cave.push([false; SIZE]);
    let mut count = 0;
    while let Some((x, y)) = next_unit(&cave) {
        cave[y][x] = true;
        count += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn example() {
        let cave = parse(EXAMPLE);
        assert_eq!(part1(&cave), 24);
        assert_eq!(part2(&cave), 93);
    }
}
