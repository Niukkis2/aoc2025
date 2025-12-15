use aoc2025::utils::day04_input;

pub fn solve_d4p1() {
    let input = day04_input("src/days/inputs/day04/day04.txt");

    let offsets: [[i32; 2]; 8] = [
        [-1, -1],
        [-1, 0],
        [-1, 1],
        [0, 1],
        [1, 1],
        [1, 0],
        [1, -1],
        [0, -1],
    ];

    let mut total: i32 = 0;

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == '.' {
                continue;
            }
            let mut count: i32 = 0;
            for xy in offsets {
                let x = i as i32 + xy[0];
                let y = j as i32 + xy[1];
                let l = input.len();
                if out_of_bounds(x, y, l) {
                    continue;
                } else {
                    if input[x as usize][y as usize] == '@' {
                        count += 1;
                    }
                }
            }
            if count < 4 {
                total += 1;
            }
        }
    }
    println!("Day4 p1: {total}");
}

pub fn solve_d4p2() {
    let mut input = day04_input("src/days/inputs/day04/day04.txt");

    let offsets: [[i32; 2]; 8] = [
        [-1, -1],
        [-1, 0],
        [-1, 1],
        [0, 1],
        [1, 1],
        [1, 0],
        [1, -1],
        [0, -1],
    ];

    let mut total: i32 = 0;
    loop {
        let prev_total: i32 = total;
        for i in 0..input.len() {
            for j in 0..input[i].len() {
                if input[i][j] == '.' {
                    continue;
                }
                let mut count: i32 = 0;
                for xy in offsets {
                    let x = i as i32 + xy[0];
                    let y = j as i32 + xy[1];
                    let l = input.len();
                    if out_of_bounds(x, y, l) {
                        continue;
                    } else {
                        if input[x as usize][y as usize] == '@' {
                            count += 1;
                        }
                    }
                }
                if count < 4 {
                    total += 1;
                    input[i][j] = '.';
                }
            }
        }
        if prev_total == total {
            break;
        }
    }
    println!("Day4 p1: {total}");
}

fn out_of_bounds(i: i32, j: i32, length: usize) -> bool {
    let leni32 = length as i32;
    i < 0 || j < 0 || i >= leni32 || j >= leni32
}
