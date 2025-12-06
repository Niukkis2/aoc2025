use aoc2025::utils::file_to_vec;

pub fn solve() {
    let input: Vec<String> = file_to_vec("src/days/inputs/day03/day03.txt");
    let mut total: u32 = 0;
    for batteries in input {
        let as_chars: Vec<char> = batteries.chars().collect();
        let as_ints: Vec<u32> = as_chars
            .into_iter()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        let mut max_first: u32 = 0;
        let mut max_first_idx: usize = 0;
        for (i, max) in as_ints.iter().enumerate() {
            if *max > max_first {
                max_first = *max;
                max_first_idx = i;
            }
        }
        let mut max_second: u32 = 0;
        let mut max_second_idx: usize = 0;
        if max_first_idx == as_ints.len() - 1 {
            for i in 0..as_ints.len() - 1 {
                if as_ints[i] > max_second {
                    max_second = as_ints[i];
                    max_second_idx = i;
                }
            }
        } else {
            for i in max_first_idx + 1..as_ints.len() {
                if as_ints[i] > max_second {
                    max_second = as_ints[i];
                    max_second_idx = i;
                }
            }
        }
        if max_first_idx < max_second_idx {
            total += max_first * 10 + max_second;
        } else {
            total += max_second * 10 + max_first;
        }
    }
    println!("Day03 P1: {total}");
}
