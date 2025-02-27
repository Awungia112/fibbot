use octocrab::Octocrab;

async fn post_comment(token: &str, pr_id: u64, comment: &str) {
    let octocrab = Octocrab::builder().personal_token(token.to_string()).build().unwrap();
    octocrab.issues("owner", "repo").create_comment(pr_id, comment).await.unwrap();
}