use aoc2025::utils::file_to_vec;

pub fn solve_d3p1() {
    let input: Vec<String> = file_to_vec("src/days/inputs/day03/day03.txt");
    let mut total: u32 = 0;
    for batteries in input {
        let as_chars: Vec<char> = batteries.chars().collect();
        let as_ints: Vec<u32> = as_chars
            .into_iter()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        let mut max_first: u32 = 0;
        let mut max_first_idx: usize = 0;
        for (i, max) in as_ints.iter().enumerate() {
            if *max > max_first {
                max_first = *max;
                max_first_idx = i;
            }
        }
        let mut max_second: u32 = 0;
        let mut max_second_idx: usize = 0;
        if max_first_idx == as_ints.len() - 1 {
            for i in 0..as_ints.len() - 1 {
                if as_ints[i] > max_second {
                    max_second = as_ints[i];
                    max_second_idx = i;
                }
            }
        } else {
            for i in max_first_idx + 1..as_ints.len() {
                if as_ints[i] > max_second {
                    max_second = as_ints[i];
                    max_second_idx = i;
                }
            }
        }
        if max_first_idx < max_second_idx {
            total += max_first * 10 + max_second;
        } else {
            total += max_second * 10 + max_first;
        }
    }
    println!("Day03 P1: {total}");
}

pub fn solve_d3p2() {
    let input: Vec<String> = file_to_vec("src/days/inputs/day03/day03.txt");
    let mut total: u64 = 0;
    for batteries in input {
        let as_chars: Vec<char> = batteries.chars().collect();
        let as_numbers: Vec<u64> = as_chars
            .into_iter()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect();
        let mut remaining_batteries: usize = as_numbers.len();
        let mut free_spaces: usize = 12;
        let mut stack: Vec<u64> = Vec::new();

        for (i, num) in as_numbers.iter().enumerate() {
            if stack.is_empty() {
                stack.push(*num);
                free_spaces -= 1;
                remaining_batteries -= 1;
                continue;
            } else if *num > stack[stack.len() - 1] {
                while let Some(&last) = stack.last() {
                    if last < *num && free_spaces < remaining_batteries {
                        stack.pop();
                        free_spaces += 1;
                    } else {
                        break;
                    }
                }

                if stack.is_empty() {
                    stack.push(*num);
                    remaining_batteries -= 1;
                    free_spaces -= 1;
                    continue;
                }
            }

            if free_spaces == remaining_batteries {
                stack.extend_from_slice(&as_numbers[i..as_numbers.len()]);
                break;
            }

            if free_spaces == 0 && (*num == stack[stack.len() - 1] || *num < stack[stack.len() - 1])
            {
                remaining_batteries -= 1;
                continue;
            } else {
                stack.push(*num);
                free_spaces -= 1;
            }

            remaining_batteries -= 1;
        }

        let num_str = stack.iter().map(|n| n.to_string()).collect::<String>();
        total += num_str.parse::<u64>().unwrap();
    }

    println!("Day3 P2: {total}")
}
