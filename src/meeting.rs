//! Meeting orchestration: emails -> titles -> salaries -> toll report.
//! Pure domain logic, independent of HTTP.

use anyhow::Result;

use crate::directory::DirectoryProvider;
use crate::salary::SalaryBook;
use crate::toll::{self, Attendee, TollReport};

/// Look up each invitee's title, price it, and total the meeting.
pub async fn compute_toll<D: DirectoryProvider>(
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

    let total = toll::round_cents(attendees.iter().map(|a| a.cost).sum());
    Ok(TollReport {
        duration_minutes,
        attendees,
        total,
    })
}
