use aoc2025::utils::day01_input;

pub fn solve() {
    let instructions = day01_input("inputs/day01.txt");
    let mut dial: i32 = 50;
    let mut zeros: i32 = 0;
    for (dir, mut num) in instructions {
        if dir == "L" {
            num = -num;
        }
        if dial == 0 && num < 0 {
            zeros -= 1;
        }
        dial += num;
        zeros += dial.div_euclid(100).abs();
        dial = dial.rem_euclid(100);
        if dial == 0 && num < 0 {
            zeros += 1;
        }
    }
    println!("Day1 Part2: {zeros}");
}
