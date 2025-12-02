use std::fs::read_to_string;

pub fn file_to_vec(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .expect("Couldn't read file")
        .lines()
        .map(|s| s.to_string())
        .collect()
}
