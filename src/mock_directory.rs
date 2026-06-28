//! In-process mock of the directory.
//! DELETE THIS FILE to go live (see the GO LIVE note in main.rs).

use anyhow::Result;

use crate::directory::{DirectoryProvider, UserInfo};

/// Mock users: (email, display name, job title)
const USERS: &[(&str, &str, &str)] = &[
    ("shiva.reddy@ulteig.com", "Shiva Reddy", "Software Engineer"),
    ("jane.doe@ulteig.com", "Jane Doe", "Project Manager"),
    ("john.smith@ulteig.com", "John Smith", "Principal Engineer"),
    ("amy.lee@ulteig.com", "Amy Lee", "Director of Engineering"),
    ("raj.patel@ulteig.com", "Raj Patel", "Senior Data Scientist"),
];

pub struct MockDirectory;

impl MockDirectory {
    pub fn new() -> Self {
        Self
    }
}

impl DirectoryProvider for MockDirectory {
    async fn get_user(&self, email: &str) -> Result<Option<UserInfo>> {
        let found = USERS.iter().find(|(e, _, _)| e.eq_ignore_ascii_case(email));
        Ok(found.map(|(_, name, title)| UserInfo {
            display_name: name.to_string(),
            job_title: title.to_string(),
        }))
    }
}
