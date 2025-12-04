use aoc2025::utils::day02_input;

pub fn solve() {
    let input: Vec<String> = day02_input("src/days/inputs/day02/day02.txt");
    let mut invalids: i64 = 0;
    for line in input {
        let splits: Vec<&str> = line.as_str().split("-").collect();
        let range_start: i64 = splits[0].parse().expect("e");
        let range_end: i64 = splits[1].parse().expect("e");
        for i in range_start..range_end + 1 {
            let num_as_str: String = i.to_string();
            let len_half: usize = num_as_str.len() - num_as_str.len() / 2;
            let halves: (&str, &str) = num_as_str.split_at(len_half);
            if halves.0 == halves.1 {
                invalids += i;
            }
        }
    }
    println!("Day2 P1: {invalids}");
}
