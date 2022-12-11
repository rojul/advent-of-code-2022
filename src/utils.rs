use std::{fs, path::Path};

pub fn read_input(day: u8) -> String {
    let path = Path::new("inputs").join(format!("{:02}.txt", day));
    let mut input = fs::read_to_string(path).unwrap();
    input.retain(|c| c != '\r');
    if input.ends_with('\n') {
        input.pop();
    }
    input
}
