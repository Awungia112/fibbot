use std::env;
use octocrab::Octocrab;
use dotenv::dotenv;
use reqwest::Client;

mod extract;
mod nacci;
mod post;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    // Read environment variables
    let enable_fib = env::var("INPUT_ENABLE_FIB").unwrap_or_else(|_| "false".to_string());
    let max_threshold = env::var("INPUT_MAX_THRESHOLD").unwrap_or_else(|_| "100".to_string()).parse::<u64>().unwrap_or(100);
    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set");
    let repo_owner = env::var("GITHUB_REPOSITORY_OWNER").expect("GITHUB_REPOSITORY_OWNER must be set");
    let repo_name = env::var("GITHUB_REPOSITORY_NAME").expect("GITHUB_REPOSITORY_NAME must be set");
    let pr_number = env::var("GITHUB_PR_NUMBER").expect("GITHUB_PR_NUMBER must be set").parse::<u64>().unwrap();

    println!("Enable Fib: {}", enable_fib);
    println!("Max Threshold: {}", max_threshold);

    // Initialize Octocrab with the GitHub token
    let octocrab = Octocrab::builder().personal_token(github_token).build().unwrap();
    

    // Get the pull request content
    let pr = octocrab.pulls(repo_owner.clone(), repo_name.clone()).get(pr_number).await.unwrap();
    let pr_content = pr.body.as_deref().unwrap_or("");

    // Extract numbers from the pull request content
    let numbers = extract::extract(pr_content);

    // Calculate Fibonacci values for the extracted numbers
    let fibonacci_values: Vec<(u32, u64)> = numbers
        .iter()
        .filter_map(|&num| {
            if num as u64 <= max_threshold {
                Some((num, nacci::fibonacci(num as u32)))
            } else {
                None
            }
        })
        .collect();

    // Generate the Fibonacci sequence for a specified number of terms
    let num_terms = 45;
    let fibonacci_sequence = nacci::fibonacci_sequence(num_terms);

    // Format the comment
    let comment = post::format_fibonacci_values(&fibonacci_values);
    let sequence_comment = post::format_fibonacci_sequence(&fibonacci_sequence);

    // Post the comment on the pull request
    octocrab.issues(repo_owner, repo_name).create_comment(pr_number, format!("{}\n\n{}", comment, sequence_comment)).await.unwrap();
    
}
fn post_to_github(fib_numbers: &[i32]) -> Result<(), reqwest::Error> {
    let token = env::var("GITHUB_TOKEN")
        .unwrap_or_else(|_| panic!("Missing GITHUB_TOKEN environment variable"));

    let client = Client::new();
    let pr_url = env::var("GITHUB_API_URL")
        .expect("Missing GITHUB_API_URL environment variable")
        + "/repos/"
        + &env::var("GITHUB_REPOSITORY")
            .expect("Missing GITHUB_REPOSITORY environment variable")
        + "/issues/"
        + &env::var("GITHUB_REF")
            .expect("Missing GITHUB_REF environment variable")
        + "/comments";
    Ok(())
}
