//! UU PDP consent enforcement — gates PII writes on a valid DataConsent record.
//!
//! The coherence council's #2 finding (2026-08-01): a *legal precondition*, not a feature.
//! Under Indonesia's UU PDP (UU 13/2022), storing regulated PII (NIK, NPWP, religion, family,
//! bank) without a valid consent record is non-compliant. This guard checks that a valid
//! (non-withdrawn, non-expired) DataConsent exists before a PII write is allowed.
//!
//! Usage: PII write-services call `require_consent(pool, employee_id, &category)` before
//! delegating to the CRUD create/update. If no valid consent exists → rejected.

use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::entity::DataCategory;

/// Check whether a valid (non-withdrawn, non-expired, non-deleted) DataConsent exists
/// for the given employee + data category.
///
/// A valid consent:
/// - `employee_id` matches
/// - `data_category` matches
/// - `withdrawn_at IS NULL` (not withdrawn)
/// - `retention_until IS NULL OR retention_until >= CURRENT_DATE` (not expired)
/// - `(metadata->>'deleted_at') IS NULL` (not soft-deleted)
pub async fn has_valid_consent(
    pool: &PgPool,
    employee_id: Uuid,
    category: &DataCategory,
) -> bool {
    let result: Result<(bool,), sqlx::Error> = sqlx::query_as(
        r#"SELECT EXISTS(
            SELECT 1 FROM employee.data_consents
            WHERE employee_id = $1
              AND data_category = $2::data_category
              AND withdrawn_at IS NULL
              AND (retention_until IS NULL OR retention_until >= CURRENT_DATE)
              AND (metadata->>'deleted_at') IS NULL
        )"#,
    )
    .bind(employee_id)
    .bind(category.to_string())
    .fetch_one(pool)
    .await;

    result.map(|(exists,)| exists).unwrap_or(false)
}

/// Require valid consent for a PII write. Returns `Err(message)` if no valid DataConsent
/// exists for the employee + data category — the caller should reject the write.
///
/// The message identifies the category + employee for the API response / audit log.
pub async fn require_consent(
    pool: &PgPool,
    employee_id: Uuid,
    category: &DataCategory,
) -> Result<(), String> {
    if has_valid_consent(pool, employee_id, category).await {
        Ok(())
    } else {
        Err(format!(
            "UU PDP consent required: no valid DataConsent for category '{}' (employee {}). \
             Capture consent before writing this PII.",
            category, employee_id
        ))
    }
}

/// The data category for each PII entity (the mapping the write-services use).
/// Entities NOT listed here (Education, Certification, WorkExperience) are not
/// Art-16-regulated PII and are not consent-gated.
pub mod categories {
    use crate::domain::entity::DataCategory;

    pub const IDENTITY: DataCategory = DataCategory::Identity;
    pub const FINANCIAL: DataCategory = DataCategory::Financial;
    pub const FAMILY: DataCategory = DataCategory::Family;
    pub const CONTACT: DataCategory = DataCategory::Contact;
}
