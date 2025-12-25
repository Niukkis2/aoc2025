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
    println!("d5 p1: {}", fresh);
}

pub fn solve_d5p2() {
    let (ranges, _values) = day05_input("src/days/inputs/day05/day05.txt");
    let mut start_end: Vec<(i64, i64)> = ranges
        .iter()
        .map(|s| {
            let mut parts = s.split('-');
            let start = parts.next().unwrap().parse::<i64>().unwrap();
            let end = parts.next().unwrap().parse::<i64>().unwrap();
            (start, end)
        })
        .collect();
    start_end.sort_by(|a, b| a.partial_cmp(b).unwrap());

    println!("{:?}", start_end);

    let mut merged: Vec<(i64, i64)> = Vec::new();
    for range in start_end {
        if let Some(last) = merged.last_mut() {
            if range.0 <= last.1 + 1 {
                last.1 = last.1.max(range.1);
            } else {
                merged.push(range);
            }
        } else {
            merged.push(range);
        }
    }

    let fresh: i64 = merged.iter().map(|(start, end)| end - start + 1).sum();

    println!("d5 p2: {}", fresh);
}
