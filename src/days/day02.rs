use aoc2025::utils::day02_input;

pub fn solve_d2p1() {
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

pub fn solve_d2p2() {
    let input: Vec<String> = day02_input("src/days/inputs/day02/day02.txt");
    let mut invalids: i64 = 0;
    for line in input {
        let splits: Vec<&str> = line.as_str().split("-").collect();
        let range_start: i64 = splits[0].parse().expect("e");
        let range_end: i64 = splits[1].parse().expect("e");
        for i in range_start..range_end + 1 {
            let s = i.to_string();
            let len_num: i32 = s.len() as i32; //len_num: correct
            let factors: Vec<i32> = find_factors(len_num);
            if every_char_same(i) && len_num > 1 {
                invalids += i;
                continue;
            }
            for factor in factors {
                let chunks: Vec<&str> = s
                    .as_bytes()
                    .chunks(factor as usize)
                    .map(|chunk| std::str::from_utf8(chunk).unwrap())
                    .collect();
                if compare_strs(chunks) {
                    invalids += i;
                    break;
                }
            }
        }
    }
    println!("Day2 P2: {invalids}");
}

fn compare_strs(chunks: Vec<&str>) -> bool {
    for i in 1..chunks.len() {
        if chunks[i] != chunks[i - 1] {
            return false;
        }
    }
    true
}

fn every_char_same(num: i64) -> bool {
    let chars: Vec<char> = num.to_string().chars().collect();
    for i in 1..chars.len() {
        if chars[i] != chars[i - 1] {
            return false;
        }
    }
    true
}

fn find_factors(num: i32) -> Vec<i32> {
    let mut factors: Vec<i32> = Vec::new();
    for i in 2..num {
        if num % i == 0 {
            factors.push(i);
        }
    }
    factors
}
