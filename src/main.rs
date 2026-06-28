// GOAL: This server, that plugin can ask toll for a given list of users

use anyhow::Result;
use reqwest::ClientBuilder;
use std::time::Duration;

const GRAPH_TOKEN: &str = "alksjdf"; // later moved to .env file

#[tokio::main]
async fn main() -> Result<()> {
    println!("Meeting Toll");

    // Testing Microsoft Graph API
    println!("Testing Microsoft Graph API");

    // let access_token = std::env::var("GRAPH_TOKEN")?;
    let access_token = GRAPH_TOKEN;
    let user_email = "shiva.reddy@ulteig.com"; // test user email

    let info = get_user_jobtitle(&access_token, user_email).await?;
    println!("{info}");

    Ok(())
}

// Calls the Microsoft Graph API for a user's display name and job title.
async fn get_user_jobtitle(access_token: &str, email: &str) -> Result<String> {
    let url =
        format!("https://graph.microsoft.com/v1.0/users/{email}?$select=displayName,jobTitle");

    let timeout = Duration::new(5, 0);
    let client = ClientBuilder::new().timeout(timeout).build()?;

    let resp = client.get(&url).bearer_auth(access_token).send().await?;

    if !resp.status().is_success() {
        anyhow::bail!("Graph API returned an error: {}", resp.status());
    }

    let body = resp.text().await?;
    Ok(body)
}
