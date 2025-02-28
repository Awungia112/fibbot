pub fn extract(content: &str) -> Vec<u32> {
    content.split_whitespace()
        .filter_map(|word| word.parse().ok())
        .collect()
}