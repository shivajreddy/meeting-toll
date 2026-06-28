// GOAL: This server, that plugin can ask toll for a given list of users

mod directory;
mod meeting;
mod mock_directory;
mod salary;
mod server;
mod toll;

use anyhow::Result;

use mock_directory::MockDirectory;
use salary::SalaryBook;

#[tokio::main]
async fn main() -> Result<()> {
    // Directory (where titles come from).
    // GO LIVE: delete src/mock_directory.rs, drop the two `mock_directory` lines,
    // and replace the next line with:
    //     let dir = directory::GraphDirectory::new(std::env::var("GRAPH_TOKEN")?)?;
    let dir = MockDirectory::new();

    // Salary book (the admin's Excel - hardcoded for now).
    let book = SalaryBook::sample();

    server::run(dir, book).await
}
