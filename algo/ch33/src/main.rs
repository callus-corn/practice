use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    println!("{input}");

    let input_value = input
        .trim()
        .split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect::<Vec<i32>>();

    let mut result = [256, 257];

    for v in input_value {
        let mut bubble = v;
        for i in 0..result.len() {
            if bubble < result[i] {
                let tmp = result[i];
                result[i] = bubble;
                bubble = tmp;
            }
        }
    }

    let second = result[1];
    println!("{second}");

    Ok(())
}
