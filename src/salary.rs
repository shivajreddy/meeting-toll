//! The salary book: two tables an admin maintains (today hardcoded; later an Excel file).
//!   - title_salary:   exact job title    -> annual salary
//!   - bracket_salary: seniority keyword  -> annual salary  (priority order, first match wins)
//!
//! Resolution rule: exact title first, then bracket fallback.

pub struct SalaryBook {
    title_salary: Vec<(String, u32)>,
    bracket_salary: Vec<(String, u32)>,
}

impl SalaryBook {
    /// Hardcoded sample standing in for the admin's Excel file.
    /// TODO: replace with `SalaryBook::from_xlsx(path)` (calamine) once the format is final.
    pub fn sample() -> Self {
        // Exact job title -> annual salary.
        let title_salary = own_rows(vec![
            ("Software Engineer", 115_000),
            ("Project Manager", 130_000),
            ("Principal Engineer", 185_000),
            ("Director of Engineering", 210_000),
        ]);

        // Seniority keyword -> annual salary. Highest first so e.g. "Senior ..."
        // wins before a generic "Engineer".
        let bracket_salary = own_rows(vec![
            ("Chief", 400_000),
            ("Vice President", 300_000),
            ("Director", 200_000),
            ("Principal", 180_000),
            ("Manager", 150_000),
            ("Senior", 140_000),
            ("Engineer", 110_000),
            ("Analyst", 95_000),
        ]);

        Self {
            title_salary,
            bracket_salary,
        }
    }

    /// Most relevant annual salary for a title: exact match first, else bracket keyword.
    pub fn salary_for_title(&self, title: &str) -> Option<u32> {
        let t = title.trim();
        if t.is_empty() {
            return None;
        }

        // 1) Exact title (case-insensitive).
        if let Some((_, s)) = self
            .title_salary
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(t))
        {
            return Some(*s);
        }

        // 2) Bracket fallback: first keyword (in priority order) contained in the title.
        let lower = t.to_lowercase();
        self.bracket_salary
            .iter()
            .find(|(kw, _)| lower.contains(&kw.to_lowercase()))
            .map(|(_, s)| *s)
    }
}

fn own_rows(rows: Vec<(&str, u32)>) -> Vec<(String, u32)> {
    rows.into_iter().map(|(k, v)| (k.to_string(), v)).collect()
}
