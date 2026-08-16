//! Read port for the `employee.employees_public` directory view (hr.employee.public port,
//! Wave 1 P1 / H-1).
//!
//! User-owned (declared under `user_owned` in `metaphor.codegen.yaml`). The view itself is
//! hand-authored SQL in `migrations/20260816150000_employees_public_view.up.sql` — it is NOT
//! a schema-derived entity, so nothing about it is generated. This file holds the only SQL
//! that reads it (4-layer rule: services orchestrate, repositories hold SQL).
//!
//! The view is `security_invoker = on`, so these reads run under the caller's RLS fence
//! (ADR-0008/ADR-0014): a session sees only its company's rows without any explicit
//! company predicate. Redaction is structural — the PII columns are not projected by the
//! view at all, so no query against it can leak them.

use chrono::NaiveDate;
use uuid::Uuid;
use sqlx::PgPool;

use backbone_orm::company_scope;

use crate::domain::entity::EmploymentStatus;

/// One directory row — exactly the view's column set, nothing more.
///
/// `employment_*` fields are NULL when the employee has no active employment (the view's
/// LEFT JOIN LATERAL), which is the correct peer-facing answer for a pre-onboarding row.
#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct EmployeePublicRow {
    pub id: Uuid,
    pub company_id: Uuid,
    pub employee_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub mobile_phone: Option<String>,
    pub phone: Option<String>,
    pub department_id: Option<Uuid>,
    pub level_id: Option<Uuid>,
    pub position_id: Option<Uuid>,
    pub direct_manager_id: Option<Uuid>,
    pub employment_status: Option<EmploymentStatus>,
    pub join_date: Option<NaiveDate>,
    pub end_join_date: Option<NaiveDate>,
}

/// Directory read port over `employee.employees_public`.
///
/// Redaction contract — what the view EXCLUDES and this port can therefore never return:
/// birth_place, birth_date, gender, marital_status, blood_type, religion_id, user_id.
/// Any future column added to `employees` joins that checklist at review time: project it
/// only if peers need it to reach or place a colleague.
pub struct EmployeePublicRepository;

impl EmployeePublicRepository {
    pub fn new() -> Self {
        Self
    }

    /// Page through the company directory (newest hires first — the "Newly Hired" filter
    /// posture from the Odoo directory). RLS scopes the view; `limit`/`offset` are the
    /// caller's paging contract.
    pub async fn list_public(
        &self,
        pool: &PgPool,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<EmployeePublicRow>, sqlx::Error> {
        company_scope::fetch_all_scoped(
            pool,
            sqlx::query_as(
                r#"SELECT id, company_id, employee_number, first_name, last_name,
                          email, mobile_phone, phone,
                          department_id, level_id, position_id, direct_manager_id,
                          employment_status, join_date, end_join_date
                   FROM employee.employees_public
                   ORDER BY join_date DESC NULLS LAST, employee_number
                   LIMIT $1 OFFSET $2"#,
            )
            .bind(limit)
            .bind(offset),
        )
        .await
    }

    /// Fetch one directory row by id. RLS scopes the view — another company's id is
    /// simply not matched.
    pub async fn find_public(
        &self,
        pool: &PgPool,
        id: Uuid,
    ) -> Result<Option<EmployeePublicRow>, sqlx::Error> {
        company_scope::fetch_optional_scoped(
            pool,
            sqlx::query_as(
                r#"SELECT id, company_id, employee_number, first_name, last_name,
                          email, mobile_phone, phone,
                          department_id, level_id, position_id, direct_manager_id,
                          employment_status, join_date, end_join_date
                   FROM employee.employees_public
                   WHERE id = $1"#,
            )
            .bind(id),
        )
        .await
    }
}

impl Default for EmployeePublicRepository {
    fn default() -> Self {
        Self::new()
    }
}
