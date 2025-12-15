use aoc2025::utils::day05_input;

pub fn solve_d5p1() {
    let (ranges, values) = day05_input("src/days/inputs/day05/day05.txt");
    let mut fresh: usize = 0;
    for value in values {
        for included in &ranges {
            let splits: Vec<&str> = included.split("-").collect();
            let (range_start, range_end): (i64, i64) =
                (splits[0].parse().unwrap(), splits[1].parse().unwrap());
            let value_num: i64 = value.parse().unwrap();
            if value_num >= range_start && value_num <= range_end {
                fresh += 1;
                break;
            }
        }
    }
    println!("p1: {}", fresh);
}
