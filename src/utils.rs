use std::fs::read_to_string;

pub fn file_to_vec(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .expect("Couldn't read file")
        .lines()
        .map(|s| s.to_string())
        .collect()
}

pub fn day01_input(filename: &str) -> Vec<(String, i32)> {
    read_to_string(filename)
        .expect("Couldn't read file")
        .lines()
        .map(|s| {
            (
                s[0..1].to_string(),
                s[1..s.len()].parse().expect("Expected i32"),
            )
        })
        .collect()
}

pub fn day02_input(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .expect("Couldn't read file")
        .split(",")
        .map(|s| s.trim().to_string())
        .collect()
}
