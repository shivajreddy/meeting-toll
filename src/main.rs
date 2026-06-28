// GOAL: This server, that plugin can ask toll for a given list of users

mod directory;
mod mock_directory;
mod salary;
mod toll;

use anyhow::Result;

use directory::DirectoryProvider;
use mock_directory::MockDirectory;
use salary::SalaryBook;
use toll::{Attendee, TollReport};

#[tokio::main]
async fn main() -> Result<()> {
    // 1) Directory (where titles come from).
    //    GO LIVE: delete src/mock_directory.rs, drop the two `mock_directory` lines
    //    above, and replace the next line with:
    //        let dir = directory::GraphDirectory::new(std::env::var("GRAPH_TOKEN")?)?;
    let dir = MockDirectory::new();

    // 2) Salary book (the admin's Excel - hardcoded for now).
    let book = SalaryBook::sample();

    // 3) The meeting: invitees + duration.
    let emails = [
        "shiva.reddy@ulteig.com",
        "amy.lee@ulteig.com",
        "raj.patel@ulteig.com",
    ];
    let duration_minutes = 60;

    let report = compute_toll(&dir, &book, &emails, duration_minutes).await?;
    println!("{report}");

    Ok(())
}

/// Look up each invitee's title, price it, and total the meeting.
async fn compute_toll<D: DirectoryProvider>(
    dir: &D,
    book: &SalaryBook,
    emails: &[&str],
    duration_minutes: u32,
) -> Result<TollReport> {
    let mut attendees = Vec::with_capacity(emails.len());

    for &email in emails {
        let (display_name, job_title) = match dir.get_user(email).await? {
            Some(u) => (u.display_name, u.job_title),
            None => (format!("{email} (unknown)"), String::new()),
        };

        let annual_salary = book.salary_for_title(&job_title);
        let cost = annual_salary.map_or(0.0, |s| toll::cost(s, duration_minutes));

        attendees.push(Attendee {
            display_name,
            job_title,
            annual_salary,
            cost,
        });
    }

    let total = attendees.iter().map(|a| a.cost).sum();
    Ok(TollReport {
        duration_minutes,
        attendees,
        total,
    })
}
