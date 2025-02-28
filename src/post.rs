pub fn format_fibonacci_values(fibonacci_values: &[(u32, u64)]) -> String {
    let mut comment = "Fibonacci values from PR content:\n".to_string();
    for (num, fib) in fibonacci_values {
        comment.push_str(&format!("Fibonacci({}) = {}\n", num, fib));
    }
    comment
}

pub fn format_fibonacci_sequence(fibonacci_sequence: &[u64]) -> String {
    let mut comment = "Fibonacci sequence for the first 45 terms:\n".to_string();
    for (i, num) in fibonacci_sequence.iter().enumerate() {
        comment.push_str(&format!("Fibonacci({}) = {}\n", i, num));
    }
    comment
}