use itertools::Itertools;
use std::fs::read_to_string;

pub fn file_to_vec(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect()
}

pub fn day01_input(filename: &str) -> Vec<(String, i32)> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(|s| (s[0..1].to_string(), s[1..s.len()].parse().unwrap()))
        .collect()
}

pub fn day02_input(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .split(",")
        .map(|s| s.trim().to_string())
        .collect()
}

pub fn day04_input(filename: &str) -> Vec<Vec<char>> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(|s| s.trim().chars().collect())
        .collect()
}

pub fn day05_input(filename: &str) -> (Vec<String>, Vec<String>) {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(|s| s.trim().to_string())
        .collect::<Vec<String>>()
        .split(|line| line == "")
        .map(|slice| slice.to_vec())
        .collect_tuple()
        .unwrap()
}
