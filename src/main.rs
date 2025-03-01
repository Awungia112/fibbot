use std::env;
use get_pull_request::get_pr;
use nacci::fibonacci;
use dotenv::dotenv;
use post::post_comment;

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
    
    println!("Enable Fib: {}", enable_fib);
    println!("Max Threshold: {}", max_threshold);

   
    let pr_number: u64 = env::var("PR_NUMBER")
    .expect("PR_NUMBER not set")
    .parse::<u64>()
    .expect("Invalid PR_NUMBER");
    // Extract numbers from the pull request content
    let numbers = get_pr(pr_number).await;
    println!("{:?}", numbers);

    // Calculate Fibonacci values for the extracted numbers
    let mut fibonacci_values= String::from("the fibonacci are\n");
    for number in numbers {
        
        let fib = fibonacci(number);
        fibonacci_values.push_str(&format!("- Fibonacci({}) = {}\n", number, fib));
    };
    println!("values: {}", fibonacci_values);

    

    if let Err(e) = post_comment(&fibonacci_values).await {
        eprint!("Error posting comment: {}", e);
    }
    
}

