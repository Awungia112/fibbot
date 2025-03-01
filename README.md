
  
# FibBot GitHub Action

## Summary

FibBot is a GitHub Action written in Rust that scans pull request content for numbers, calculates their Fibonacci numbers, and posts a comment with the results. The action supports two parameters: a flag to enable Fibonacci calculation and a threshold limit.


## Features

- Scans pull request content for numbers.
- Calculates Fibonacci value for the extracted numbers.
- Posts a comment with the Fibonacci results.
- Supports configuration parameters for enabling/disabling calculation and setting a threshold.


## Usage

1. **Clone the Repository**
   ```sh
   git clone https://github.com/your-username/fibbot.git
   cd fibbot
2. **Build Rust Project**
   ```sh
   cargo build --release
   ```
3. **In your Github repo add this to your workflow file**
   ```
    - name: Run FibBot
      uses: Awungia112/fibbot@master
      with:
          pr_number: ${{ github.event.pull_request.number }}
          enable_fib: "true"
          max_threshold: "100"
          github_token: ${{ secrets.GITHUB_TOKEN }}
   ``` 
   
## N.B
 In your repository make sure your workflow has read and write permissions.
 
 Make sure you Github Personal Access Token is set.
 Monitor your logs to be sure that the comment is posted succesfully and the fibonacci calculated.
