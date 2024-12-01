use std::fs;

pub fn read_input(path: &str) -> String {
    let path = format!("../input/{}", path);

    fs::read_to_string(path).expect("Something went wrong reading the file")
}