use std::{io, num::ParseIntError};
use std::collections::HashMap;

const MAX_N: u32 = 75;

fn main() {
    fib_main();
}

fn fib_main() {
    println!("Hello, world! Please enter n, for which the nth fibonacci number will be computed.");

    // Declare memoization table
    let mut memo: HashMap<u32, u64> = HashMap::new();

    // Gather input
    let mut input = String::new();
    _ = io::stdin().read_line(&mut input);
    let input = input.trim();
    println!("Received `{}`", &input);

    // Parse input and compute
    let n_result: Result<u32, ParseIntError> = input.parse();
    if !n_result.is_err() {
        let num: u32 = n_result.unwrap();
        if num > MAX_N {
            println!("Error! Input must be <= {}, but received {}", MAX_N, num);
            return;
        }

        println!("{}", fib_compute(num, &mut memo));
        println!("After running that, {} elements in the memoization table", memo.len());
    } else{
        println!("Error! Message `{}`", n_result.unwrap_err());
    }
}

fn fib_compute(a: u32, memo: &mut HashMap<u32, u64>) -> u64 {
    if a <= 1 {
        return a as u64;
    }

    match memo.get(&a) {
        Some(v) => *v,
        None => {
            let new_value = fib_compute(a - 1, memo) + fib_compute(a - 2, memo);
            memo.insert(a, new_value);
            new_value
        }
    }
}
