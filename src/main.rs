use std::env;

use nacci::fibonacci;
use num::extract_numbers;

fn main() {
    let enable_fib = env::var("INPUT_ENABLE_FIB").unwrap();
    let max_threshold = env::var("INPUT_MAX_THRESHOLD").unwrap().parse::<u32>().unwrap();

    println!("Enable Fib: {}", enable_fib);
    println!("Max Threshold: {}", max_threshold);

    let numbers = extract_numbers(&pr_content);
for num in numbers {
    println!("Fibonacci({}) = {}", num, fibonacci(num));
}
}


mod num;
mod nacci;
mod post;