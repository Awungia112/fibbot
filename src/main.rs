use std::env;
use get_pull_request::get_pr;
use nacci::fibonacci;
// use octocrab::Octocrab;
use dotenv::dotenv;
use post::post_comment;
// use reqwest::Client;

mod extract;
mod nacci;
mod post;
mod get_pull_request;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    // Read environment variables
    let enable_fib = env::var("INPUT_ENABLE_FIB").unwrap_or_else(|_| "false".to_string());
    let max_threshold = env::var("INPUT_MAX_THRESHOLD").unwrap_or_else(|_| "100".to_string()).parse::<u64>().unwrap_or(100);
    // let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set");
    // let repo_owner = env::var("GITHUB_REPOSITORY_OWNER").expect("GITHUB_REPOSITORY_OWNER must be set");
    // let repo_name = env::var("GITHUB_REPOSITORY_NAME").expect("GITHUB_REPOSITORY_NAME must be set");
    // let pr_number = env::var("GITHUB_PR_NUMBER").expect("GITHUB_PR_NUMBER must be set").parse::<u64>().unwrap();

    println!("Enable Fib: {}", enable_fib);
    println!("Max Threshold: {}", max_threshold);

    // // Initialize Octocrab with the GitHub token
    // let octocrab = Octocrab::builder().personal_token(github_token).build().unwrap();
    

    // Get the pull request content
    // let pr = octocrab.pulls(repo_owner.clone(), repo_name.clone()).get(pr_number).await.unwrap();
    // // let pr_content = pr.body.as_deref().unwrap_or("");
    // let pr_number: u64 = env::var("PR_NUMBER")
    // .expect("PR_NUMBER not set")
    // .parse::<u64>()
    // .expect("Invalid PR_NUMBER");
    // Extract numbers from the pull request content
    let numbers = get_pr(2).await;
    println!("{:?}", numbers);

    // Calculate Fibonacci values for the extracted numbers
    let mut fibonacci_values= String::from("the fibonacci are\n");
    for number in numbers {
        
        let fib = fibonacci(number);
        fibonacci_values.push_str(&format!("- Fibonacci({}) = {}\n", number, fib));
    };
    println!("values: {}", fibonacci_values);

    // // Generate the Fibonacci sequence for a specified number of terms
    // let num_terms = 45;
    // let fibonacci_sequence = nacci::fibonacci_sequence(num_terms);

    if let Err(e) = post_comment(&fibonacci_values).await {
        eprint!("Error posting comment: {}", e);
    }
    
}
// fn post_to_github(fib_numbers: &[i32]) -> Result<(), reqwest::Error> {
//     let token = env::var("GITHUB_TOKEN")
//         .unwrap_or_else(|_| panic!("Missing GITHUB_TOKEN environment variable"));

//     let client = Client::new();
//     let pr_url = env::var("GITHUB_API_URL")
//         .expect("Missing GITHUB_API_URL environment variable")
//         + "/repos/"
//         + &env::var("GITHUB_REPOSITORY")
//             .expect("Missing GITHUB_REPOSITORY environment variable")
//         + "/issues/"
//         + &env::var("GITHUB_REF")
//             .expect("Missing GITHUB_REF environment variable")
//         + "/comments";
//     Ok(())
// }
