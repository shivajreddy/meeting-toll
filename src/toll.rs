//! Meeting toll = sum over attendees of (annual salary / 2080 hours) * meeting hours.

/// 40 hours/week * 52 weeks.
const WORK_HOURS_PER_YEAR: f64 = 2080.0;

#[derive(serde::Serialize)]
pub struct Attendee {
    pub display_name: String,
    pub job_title: String,
    pub annual_salary: Option<u32>,
    pub cost: f64,
}

#[derive(serde::Serialize)]
pub struct TollReport {
    pub duration_minutes: u32,
    pub attendees: Vec<Attendee>,
    pub total: f64,
}

/// Per-attendee cost for a meeting of `duration_minutes`.
pub fn cost(annual_salary: u32, duration_minutes: u32) -> f64 {
    let hourly = annual_salary as f64 / WORK_HOURS_PER_YEAR;
    round_cents(hourly * (duration_minutes as f64 / 60.0))
}

/// Round a dollar amount to whole cents.
pub fn round_cents(amount: f64) -> f64 {
    (amount * 100.0).round() / 100.0
}

impl std::fmt::Display for TollReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Meeting toll - {} min, {} attendee(s)",
            self.duration_minutes,
            self.attendees.len()
        )?;
        for a in &self.attendees {
            let salary = match a.annual_salary {
                Some(s) => format!("${}/yr", with_commas(s)),
                None => "(no salary)".to_string(),
            };
            writeln!(
                f,
                "  {:<20} {:<26} {:>14}  {:>10}",
                a.display_name,
                a.job_title,
                salary,
                format!("${:.2}", a.cost)
            )?;
        }
        writeln!(f, "  {}", "-".repeat(74))?;
        write!(
            f,
            "  {:<20} {:<26} {:>14}  {:>10}",
            "TOTAL",
            "",
            "",
            format!("${:.2}", self.total)
        )
    }
}

/// Group a positive integer with thousands separators, e.g. 115000 -> "115,000".
fn with_commas(n: u32) -> String {
    let s = n.to_string();
    let len = s.len();
    let mut out = String::with_capacity(len + len / 3);
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    out
}
