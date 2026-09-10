//! Read port for the `employee.employees_public` directory view.
//!
//! User-owned (declared under `user_owned` in `metaphor.codegen.yaml`). The view itself is
//! hand-authored SQL — created by `migrations/20260816150000_employees_public_view.up.sql` and
//! rebuilt tenant-free by the `strip_company_tenancy` migration; it is NOT a schema-derived
//! entity, so nothing about it is generated. This file holds the only SQL that reads it
//! (4-layer rule: services orchestrate, repositories hold SQL).
//!
//! The view is `security_invoker = on`, so these reads run under whatever RLS fence the
//! composing service's tenancy decorator installed (ADR-0029): a session sees only its unit's
//! rows without any explicit predicate, while an undecorated deployment (FORCE RLS, no policy)
//! sees none. Redaction is structural — the PII columns are not projected by the view at all,
//! so no query against it can leak them.

use chrono::NaiveDate;
use uuid::Uuid;
use sqlx::{PgPool, Row};

use backbone_orm::org_scope;

use crate::domain::entity::EmploymentStatus;

/// One directory row — exactly the view's column set, nothing more.
///
/// `employment_*` fields are NULL when the employee has no active employment (the view's
/// LEFT JOIN LATERAL), which is the correct peer-facing answer for a pre-onboarding row.
#[derive(Debug, Clone, serde::Serialize)]
pub struct EmployeePublicRow {
    pub id: Uuid,
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

    /// Page through the directory (newest hires first — the "Newly Hired" filter posture from
    /// the Odoo directory). The ambient org scope fences the view; `limit`/`offset` are the
    /// caller's paging contract.
    pub async fn list_public(
        &self,
        pool: &PgPool,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<EmployeePublicRow>, sqlx::Error> {
        // (backbone_orm's org scope offers no fetch-all helper, so the scoped path is a short
        // read-only transaction here.)
        let query = sqlx::query(
            r#"SELECT id, employee_number, first_name, last_name,
                      email, mobile_phone, phone,
                      department_id, level_id, position_id, direct_manager_id,
                      employment_status, join_date, end_join_date
               FROM employee.employees_public
               ORDER BY join_date DESC NULLS LAST, employee_number
               LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset);
        let rows = if let Some(scope) = org_scope::current_org_scope() {
            let mut tx = pool.begin().await?;
            org_scope::bind_org_scope_on(&mut tx, &scope).await?;
            let rows = query.fetch_all(&mut *tx).await?;
            tx.commit().await?; // read-only: nothing but the relayed scope to close out
            rows
        } else {
            query.fetch_all(pool).await?
        };
        Ok(rows
            .into_iter()
            .map(|r| EmployeePublicRow {
                id: r.get("id"),
                employee_number: r.get("employee_number"),
                first_name: r.get("first_name"),
                last_name: r.get("last_name"),
                email: r.get("email"),
                mobile_phone: r.get("mobile_phone"),
                phone: r.get("phone"),
                department_id: r.get("department_id"),
                level_id: r.get("level_id"),
                position_id: r.get("position_id"),
                direct_manager_id: r.get("direct_manager_id"),
                employment_status: r.get("employment_status"),
                join_date: r.get("join_date"),
                end_join_date: r.get("end_join_date"),
            })
            .collect())
    }

    /// Fetch one directory row by id. The ambient org scope fences the view — another unit's
    /// id is simply not matched.
    pub async fn find_public(
        &self,
        pool: &PgPool,
        id: Uuid,
    ) -> Result<Option<EmployeePublicRow>, sqlx::Error> {
        let row = org_scope::fetch_optional_row_scoped(
            pool,
            sqlx::query(
                r#"SELECT id, employee_number, first_name, last_name,
                          email, mobile_phone, phone,
                          department_id, level_id, position_id, direct_manager_id,
                          employment_status, join_date, end_join_date
                   FROM employee.employees_public
                   WHERE id = $1"#,
            )
            .bind(id),
        )
        .await?;
        Ok(row.map(|r| EmployeePublicRow {
            id: r.get("id"),
            employee_number: r.get("employee_number"),
            first_name: r.get("first_name"),
            last_name: r.get("last_name"),
            email: r.get("email"),
            mobile_phone: r.get("mobile_phone"),
            phone: r.get("phone"),
            department_id: r.get("department_id"),
            level_id: r.get("level_id"),
            position_id: r.get("position_id"),
            direct_manager_id: r.get("direct_manager_id"),
            employment_status: r.get("employment_status"),
            join_date: r.get("join_date"),
            end_join_date: r.get("end_join_date"),
        }))
    }
}

impl Default for EmployeePublicRepository {
    fn default() -> Self {
        Self::new()
    }
}
