use crate::lib::file_to_vec;

pub fn solve() {
    let instructions = file_to_vec("inputs/day01test.txt");
    let mut dial: i32 = 50;
    for ins in instructions {
        println!("instruction: {}", ins);
        let slice: &str = &ins[0..1];
        println!("slice: {}", slice);
    }
}
