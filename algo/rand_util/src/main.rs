use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    for _ in 0..5 {
        let x: u8 = rng.gen();
        print!("{x} ");
    }
    print!("\n");
}
