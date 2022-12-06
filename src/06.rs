mod utils;

fn main() {
    let chars = utils::read_input(6).into_bytes();

    println!("{}", chars.windows(4).position(is_unique).unwrap() + 4);
    println!("{}", chars.windows(14).position(is_unique).unwrap() + 14);
}

fn is_unique(chars: &[u8]) -> bool {
    !(0..chars.len() - 1).any(|i| chars[i + 1..].contains(&chars[i]))
}
