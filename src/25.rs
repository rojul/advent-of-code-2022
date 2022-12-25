mod utils;

fn main() {
    let input = utils::read_input(25);

    println!("{}", to_snafu(input.lines().map(to_decimal).sum()));
}

const CHARS: &[u8] = b"=-012";

fn to_decimal(snafu: &str) -> i64 {
    let mut decimal = 0;
    for char in snafu.bytes() {
        decimal *= 5;
        decimal += CHARS.iter().position(|&c| c == char).unwrap() as i64 - 2;
    }
    decimal
}

fn to_snafu(mut decimal: i64) -> String {
    let mut snafu = Vec::new();
    while decimal != 0 {
        decimal += 2;
        snafu.push(CHARS[(decimal % 5) as usize]);
        decimal /= 5;
    }
    snafu.reverse();
    String::from_utf8(snafu).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        for (decimal, snafu) in [
            (906, "12111"),
            (1747, "1=-0-2"),
            (2022, "1=11-2"),
            (12345, "1-0---0"),
            (314159265, "1121-1110-1=0"),
        ] {
            assert_eq!(to_snafu(decimal), snafu, "to_snafu {decimal}");
            assert_eq!(to_decimal(snafu), decimal, "to_decimal {snafu}");
        }
    }
}
