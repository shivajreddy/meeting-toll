//! Directory providers: look up a user's name + job title by email.
#![allow(dead_code)] // GraphDirectory is the go-live path, unused until the mock is removed.

use std::future::Future;

use anyhow::Result;

/// What we need to know about a person to price a meeting.
pub struct UserInfo {
    pub display_name: String,
    pub job_title: String,
}

/// Source of user titles. Swap implementations to go from mock -> real Graph.
/// The `Send` bound lets the future run across threads, which axum handlers require.
pub trait DirectoryProvider {
    /// Returns `None` if the user doesn't exist.
    fn get_user(&self, email: &str) -> impl Future<Output = Result<Option<UserInfo>>> + Send;
}

/// Real Microsoft Graph implementation (used once admin consent is granted).
pub struct GraphDirectory {
    client: reqwest::Client,
    access_token: String,
    base_url: String,
}

impl GraphDirectory {
    pub fn new(access_token: String) -> Result<Self> {
        let client = reqwest::ClientBuilder::new()
            .timeout(std::time::Duration::from_secs(10))
            .build()?;
        // Override only needed for national clouds / test endpoints.
        let base_url = std::env::var("GRAPH_BASE_URL")
            .unwrap_or_else(|_| "https://graph.microsoft.com/v1.0".to_string());
        Ok(Self {
            client,
            access_token,
            base_url,
        })
    }
}

#[derive(serde::Deserialize)]
struct GraphUser {
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    #[serde(rename = "jobTitle")]
    job_title: Option<String>,
}

impl DirectoryProvider for GraphDirectory {
    async fn get_user(&self, email: &str) -> Result<Option<UserInfo>> {
        let url = format!(
            "{}/users/{email}?$select=displayName,jobTitle",
            self.base_url
        );
        let resp = self
            .client
            .get(&url)
            .bearer_auth(&self.access_token)
            .send()
            .await?;

        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }
        if !resp.status().is_success() {
            anyhow::bail!("Graph API returned an error: {}", resp.status());
        }

        let u: GraphUser = resp.json().await?;
        Ok(Some(UserInfo {
            display_name: u.display_name.unwrap_or_default(),
            job_title: u.job_title.unwrap_or_default(),
        }))
    }
}
