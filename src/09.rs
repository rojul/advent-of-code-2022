mod utils;

use std::collections::HashSet;

fn main() {
    let input = utils::read_input(9);

    println!("{}", simulate(&input, 2));
    println!("{}", simulate(&input, 10));
}

fn simulate(input: &str, len: usize) -> usize {
    let motions = input.lines().map(|line| {
        let (dir, n) = line.split_once(' ').unwrap();
        let dir = match dir {
            "U" => (0, -1),
            "R" => (1, 0),
            "D" => (0, 1),
            "L" => (-1, 0),
            _ => panic!(),
        };
        (dir, n.parse::<u32>().unwrap())
    });

    let mut knots = vec![(0i32, 0i32); len];
    let mut visited = HashSet::new();
    visited.insert((0, 0));
    for (dir, n) in motions {
        for _ in 0..n {
            knots[0].0 += dir.0;
            knots[0].1 += dir.1;
            for i in 1..knots.len() {
                let prev = knots[i - 1];
                let curr = &mut knots[i];
                let delta_x = prev.0 - curr.0;
                let delta_y = prev.1 - curr.1;
                if delta_x.abs() > 1 || delta_y.abs() > 1 {
                    curr.0 += delta_x.signum();
                    curr.1 += delta_y.signum();
                }
            }
            visited.insert(knots[knots.len() - 1]);
        }
    }
    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const EXAMPLE2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn example1_knots2() {
        assert_eq!(simulate(EXAMPLE1, 2), 13);
    }

    #[test]
    fn example1_knots10() {
        assert_eq!(simulate(EXAMPLE1, 10), 1);
    }

    #[test]
    fn example2_knots10() {
        assert_eq!(simulate(EXAMPLE2, 10), 36);
    }
}
