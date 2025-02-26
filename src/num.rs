pub fn extract_numbers(text: &str) -> Vec<u32> {
    text
        .split_whitespace()
        .filter_map(|word| word.parse::<u32>().ok())
        .collect()
}