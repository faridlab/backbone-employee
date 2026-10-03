//! The employee number a new hire receives: the next one in the company's sequence.
//!
//! A number reads `{prefix}{zero-padded counter}` — `E000001`, `E000002`, … by default. The
//! counter is the highest one already issued under the same prefix, plus one. "Already issued"
//! means every row the transaction can see, soft-deleted ones included (they still hold their
//! number), so under the composing service's tenancy fence the sequence runs per company and an
//! unfenced deployment numbers its whole database.
//!
//! The format is a setting, not a constant: the module asks an [`EmployeeNumberFormatSource`]
//! port, which the composing service backs with its parameter store, and falls back to
//! [`EmployeeNumberFormat::default`] when no source is wired.
//!
//! Concurrency: allocation takes a transaction-scoped advisory lock first, so two hires landing
//! at once serialize on the read of the highest number instead of racing to the same one. The
//! lock releases with the transaction that inserts the employee.
//!
//! This is a user-owned custom file — it is NEVER regenerated.

use async_trait::async_trait;
use sqlx::PgConnection;

/// The default prefix, matching the numbers the seeded workforce already carries.
pub const DEFAULT_EMPLOYEE_NUMBER_PREFIX: &str = "E";
/// The default counter width (`E000001`).
pub const DEFAULT_EMPLOYEE_NUMBER_DIGITS: usize = 6;

/// The column budget for `employee.employees.employee_number`.
const NUMBER_MAX_LEN: usize = 40;
/// Bounds that keep a mistyped setting from producing an unusable number.
const PREFIX_MAX_LEN: usize = 16;
const DIGITS_MIN: usize = 1;
const DIGITS_MAX: usize = 12;

/// How an employee number is written: a literal prefix and a zero-padded counter width.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmployeeNumberFormat {
    prefix: String,
    digits: usize,
}

impl Default for EmployeeNumberFormat {
    fn default() -> Self {
        Self {
            prefix: DEFAULT_EMPLOYEE_NUMBER_PREFIX.to_string(),
            digits: DEFAULT_EMPLOYEE_NUMBER_DIGITS,
        }
    }
}

impl EmployeeNumberFormat {
    /// Build a format from settings, keeping each part only when it is usable: a prefix of up to
    /// 16 ASCII letters, digits, `-`, `_`, `/` or `.` (it is matched literally), and a width of
    /// 1–12 digits. An unusable part falls back to its default rather than failing the hire.
    pub fn new(prefix: Option<&str>, digits: Option<usize>) -> Self {
        let d = Self::default();
        let prefix = prefix
            .map(str::trim)
            .filter(|p| {
                !p.is_empty()
                    && p.len() <= PREFIX_MAX_LEN
                    && p
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '/' | '.'))
            })
            .map(str::to_string)
            .unwrap_or(d.prefix);
        let digits = digits
            .filter(|n| (DIGITS_MIN..=DIGITS_MAX).contains(n))
            .unwrap_or(d.digits);
        Self { prefix, digits }
    }

    /// The literal prefix.
    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    /// The zero-padded counter width.
    pub fn digits(&self) -> usize {
        self.digits
    }

    /// Write counter `n` in this format (`render(7)` → `E000007`). A counter wider than the
    /// width simply grows — the sequence never wraps.
    pub fn render(&self, n: i64) -> String {
        format!("{}{:0width$}", self.prefix, n, width = self.digits)
    }
}

/// Where the employee number format comes from. The composing service implements this over its
/// settings store; reads that fail should answer the default rather than block a hire.
#[async_trait]
pub trait EmployeeNumberFormatSource: Send + Sync {
    /// The format the next number is written in.
    async fn employee_number_format(&self) -> EmployeeNumberFormat;
}

/// The source used when the composer wires none: always the default format.
pub struct DefaultEmployeeNumberFormat;

#[async_trait]
impl EmployeeNumberFormatSource for DefaultEmployeeNumberFormat {
    async fn employee_number_format(&self) -> EmployeeNumberFormat {
        EmployeeNumberFormat::default()
    }
}

/// Allocate the next employee number on `conn`, which must be the transaction that will insert
/// the employee (the advisory lock is held until it ends).
///
/// Only numbers that are exactly `{prefix}{digits}` count toward the sequence, so hand-entered or
/// legacy numbers in another shape neither block nor skew it.
pub async fn allocate_employee_number(
    conn: &mut PgConnection,
    format: &EmployeeNumberFormat,
) -> Result<String, sqlx::Error> {
    sqlx::query("SELECT pg_advisory_xact_lock(hashtext('employee.employee_number'))")
        .execute(&mut *conn)
        .await?;

    let prefix_len = format.prefix().chars().count() as i32;
    let highest: i64 = sqlx::query_scalar(
        r#"SELECT COALESCE(MAX(substr(employee_number, $2 + 1)::bigint), 0)
             FROM employee.employees
            WHERE left(employee_number, $2) = $1
              AND substr(employee_number, $2 + 1) ~ '^[0-9]{1,18}$'"#,
    )
    .bind(format.prefix())
    .bind(prefix_len)
    .fetch_one(&mut *conn)
    .await?;

    let number = format.render(highest + 1);
    if number.len() > NUMBER_MAX_LEN {
        return Err(sqlx::Error::Protocol(format!(
            "employee number {number} exceeds the {NUMBER_MAX_LEN}-character column"
        )));
    }
    Ok(number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_matches_the_seeded_numbers() {
        let f = EmployeeNumberFormat::default();
        assert_eq!(f.render(1), "E000001");
        assert_eq!(f.render(1001), "E001001");
    }

    #[test]
    fn a_counter_wider_than_the_width_grows_instead_of_wrapping() {
        let f = EmployeeNumberFormat::new(Some("EMP-"), Some(3));
        assert_eq!(f.render(42), "EMP-042");
        assert_eq!(f.render(1234), "EMP-1234");
    }

    #[test]
    fn unusable_settings_fall_back_to_the_default_parts() {
        let f = EmployeeNumberFormat::new(Some("E%' OR 1=1"), Some(0));
        assert_eq!(f, EmployeeNumberFormat::default());
        let f = EmployeeNumberFormat::new(Some("   "), Some(40));
        assert_eq!(f, EmployeeNumberFormat::default());
        let f = EmployeeNumberFormat::new(Some(" HP/ "), None);
        assert_eq!(f.prefix(), "HP/");
        assert_eq!(f.digits(), DEFAULT_EMPLOYEE_NUMBER_DIGITS);
    }
}
