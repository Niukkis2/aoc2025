use aoc2025::utils::file_to_vec;

pub fn solve(p2: bool) {
    let instructions = file_to_vec("inputs/day01.txt");
    let mut dial: i32 = 50;
    let mut pw: i32 = 0;
    for instruction in instructions {
        let dir: &str = &instruction[0..1];
        let amount: i32 = instruction[1..instruction.len()]
            .trim()
            .parse()
            .expect("Expected i32");
        if p2 {
            pw += count_passes(dial, amount, dir);
            // println!("Dir: {dir}, Amount: {amount}, Dial: {dial}");
        }
        dial = match dir {
            "L" => ((dial - amount + 100) % 100 + 100) % 100,
            "R" => (dial + amount) % 100,
            _ => 0,
        };
        println!("Dir: {dir}, Amount: {amount}, Dial: {dial}, pw: {pw}");
        if dial == 0 {
            pw += 1;
        }
    }
    if p2 {
        println!("Day1 Part2: {pw}");
    } else {
        println!("Day1 Part1: {pw}");
    }
}

fn count_passes(mut dial: i32, mut amount: i32, dir: &str) -> i32 {
    let mut passes = 0;
    while amount >= 0 {
        if amount >= 100 {
            passes += 1;
            amount -= 100;
        } else {
            match dir {
                "L" => {
                    dial -= 1;
                    if dial == -1 {
                        passes += 1;
                        dial = 99;
                    }
                }
                "R" => {
                    dial += 1;
                    if dial == 100 {
                        passes += 1;
                        dial = 1;
                    }
                }
                _ => {}
            }
            amount -= 1;
        }
    }
    passes
}
