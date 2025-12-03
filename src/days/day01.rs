use aoc2025::utils::day01_input;

pub fn solve() {
    let instructions = day01_input("inputs/day01test.txt");
    let mut dial: i32 = 50;
    let mut p2: i32 = 0;
    for (dir, amount) in instructions {
        dial = match dir.as_str() {
            "L" => (dial - amount + 100) % 100,
            "R" => (dial + amount) % 100,
            _ => 0,
        };
        if dial == 0 {
            p2 += 1;
        }
        println!("Dir: {dir}, Amount: {amount}, Dial: {dial}, p2: {p2}");
    }
    println!("Day1 Part2: {p2}");
}
