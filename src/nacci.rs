pub fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

pub fn fibonacci_sequence(num_terms: u32) -> Vec<u64> {
    (0..num_terms).map(|i| fibonacci(i)).collect()
}