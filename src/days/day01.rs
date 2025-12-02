use aoc2025::utils::file_to_vec;

pub fn solve() {
    let instructions = file_to_vec("inputs/day01.txt");
    let mut dial: i32 = 50;
    let mut pw: i32 = 0;
    for instruction in instructions {
        let dir: &str = &instruction[0..1];
        let amount: i32 = instruction[1..instruction.len()]
            .trim()
            .parse()
            .expect("Expected i32");
        dial = match dir {
            "L" => move_dial_left(dial, amount),
            "R" => move_dial_right(dial, amount),
            _ => 0,
        };
        // println!("Dir: {dir}, Amount: {amount}, Dial: {dial}");
        if dial == 0 {
            pw = pw + 1;
        }
    }
    println!("Day1 Part1: {pw}");
}

fn move_dial_right(dial: i32, mut amount: i32) -> i32 {
    if amount >= 100 {
        amount = amount % 100;
    }
    let rotation: i32 = dial + amount;
    if rotation >= 100 {
        rotation - 100
    } else {
        rotation
    }
}

fn move_dial_left(dial: i32, mut amount: i32) -> i32 {
    if amount >= 100 {
        amount = amount % 100;
    }
    let rotation: i32 = dial - amount;
    if rotation < 0 {
        rotation + 100
    } else {
        rotation
    }
}
