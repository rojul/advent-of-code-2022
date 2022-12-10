mod utils;

fn main() {
    let (values, _) = exec(&utils::read_input(10));

    println!("{}", part1(&values));
    println!("\n{}\n", part2(&values));
}

fn exec(input: &str) -> (Vec<i32>, i32) {
    let instructions = input.lines().map(|line| {
        let instruction = line
            .split_once(' ')
            .map_or((line, None), |(a, b)| (a, Some(b)));
        match instruction {
            ("noop", None) => None,
            ("addx", Some(value)) => Some(value.parse::<i32>().unwrap()),
            _ => panic!(),
        }
    });

    let mut x = 1;
    let mut values = vec![];
    for instruction in instructions {
        match instruction {
            None => values.push(x),
            Some(value) => {
                values.push(x);
                values.push(x);
                x += value;
            }
        }
    }
    (values, x)
}

fn part1(values: &[i32]) -> i32 {
    [20, 60, 100, 140, 180, 220]
        .into_iter()
        .map(|cycle| cycle as i32 * values[cycle - 1])
        .sum()
}

fn part2(values: &[i32]) -> String {
    let mut screen = String::new();
    // both positions 0 instead of 1 indexed
    for (i, &sprite_position) in values.iter().enumerate() {
        let pixel_position = i as i32 % 40;
        if pixel_position == 0 && i != 0 {
            screen.push('\n');
        }
        let lit = (sprite_position - pixel_position).abs() <= 1;
        screen.push(if lit { '#' } else { '.' });
    }
    screen
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_SMALL: &str = "noop
addx 3
addx -5";

    const EXAMPLE: &str = "addx 15
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
noop";

    #[test]
    fn example_small() {
        let (values, x) = exec(EXAMPLE_SMALL);
        assert_eq!(values, [1, 1, 1, 4, 4]);
        assert_eq!(x, -1);
    }

    #[test]
    fn example_part1() {
        let (values, _) = exec(EXAMPLE);
        assert_eq!(values[20 - 1], 21);
        assert_eq!(values[60 - 1], 19);
        assert_eq!(values[100 - 1], 18);
        assert_eq!(part1(&values), 13140);
    }

    #[test]
    fn example_part2() {
        let (values, _) = exec(EXAMPLE);
        assert_eq!(
            part2(&values),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }
}
