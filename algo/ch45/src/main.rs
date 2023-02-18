
use std::error::Error;

fn check_sum(input: &[u32], target: u32) -> bool {
    if input.len() == 1 {
        return input[0] == target
    }


    let (left, right) = input.split_at(1);
    if check_sum(right, target) {
        return true
    }
    if check_sum(right, target - left[0]) {
        return true
    }

    false
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_value = [100, 100, 200, 170, 30];
    let w: u32 = 500;
    let check: bool = check_sum(&input_value, w);
    println!("{check}");
    Ok(())
}
