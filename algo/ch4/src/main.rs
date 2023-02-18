
fn fib(n: u32, memo: &mut Vec<u32>) -> u32 {
    println!("{n}");    
    if n == 0 {
        return 0
    }
    if n == 1 {
        return 1
    }

    if memo[n as usize] != 0 {
        return memo[n as usize]
    }

    memo[n as usize] = fib(n-1, memo) + fib(n-2, memo);
    memo[n as usize]
}

fn main() {
    let n: u32 = 40;
    let mut memo = vec![0; (n+1) as usize];
    let x = fib(n, &mut memo);
    println!("{x}");
}
