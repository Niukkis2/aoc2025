use aoc2025::utils::day06_input;

pub fn solve_d6p1() {
    let input = day06_input("src/days/inputs/day06/day06.txt");

    let mut total = 0;

    for j in 0..input[0].len() {
        let mut op = "";
        let mut result: i64 = 0;

        for i in (0..input.len()).rev() {
            if i == 4 {
                op = &input[i][j];
            } else {
                let next = input[i][j].parse::<i64>().unwrap();
                result = match_op(result, op, next);
            }
        }
        total += result;
    }
    println!("P1: {total}");
}

fn match_op(mut result: i64, op: &str, next: i64) -> i64 {
    match op {
        "*" => {
            if result == 0 {
                result += next
            } else {
                result *= next
            }
        }
        "+" => result += next,
        &_ => (),
    }
    result
}

pub fn solve_d6p2() {
    let mut total: i64 = 0;
    println!("P2: {total}")
}

fn process_calculation(expression: Vec<String>, sign: String) -> i64 {
    expression
        .iter()
        .map(|s| s.parse::<i64>().unwrap())
        .reduce(|prev, next| match expression.last().unwrap().as_str() {
            "*" => prev * next,
            "+" => prev + next,
            &_ => 0,
        })
        .unwrap()
}
