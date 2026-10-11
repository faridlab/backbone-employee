//! The module's extension (hand-written; ADR-0031): what the module adds beside its
//! generated services. The generated module and builder carry `ModuleExt` and
//! `ModuleBuilderExt` and dereference to them, so their fields read as `module.field`.

#[allow(unused_imports)]
use super::*;

/// State the module adds; read through the generated module's `Deref`.
pub struct ModuleExt {
    // Held so the `EmployeeQueryService` impl can delegate `employee_ptkp` to the family/tax repos'
    // hand-written SQL, and standard lookups to the CRUD services above. `db_pool` is the same pool
    // the repos were constructed with (the repo methods take it per the backbone-hr read-port
    // convention — org scoping is applied inside via `org_scope::fetch_optional_row_scoped`).
    pub(crate) employee_family_repository: Arc<EmployeeFamilyRepository>,
    pub(crate) employee_tax_repository: Arc<EmployeeTaxRepository>,
    pub(crate) db_pool: sqlx::PgPool,
    /// The self-service record-change lifecycle (submit/apply/refuse/cancel
    /// over the approvals seam) — user-owned, not schema-derived.
    pub(crate) record_change_service: std::sync::Arc<
        crate::application::service::record_change_service::RecordChangeService,
    >,
}

/// State the builder adds.
pub struct ModuleBuilderExt {
}

impl Default for ModuleBuilderExt {
    fn default() -> Self {
        Self {
        }
    }
}

impl ModuleBuilderExt {
    /// Build the extension's state from what the generated build made.
    #[allow(unused_variables, clippy::redundant_clone)]
    pub(crate) fn build(self, parts: &ModuleParts<'_>) -> anyhow::Result<ModuleExt> {
        let db_pool = parts.db_pool.clone();
        let employee_family_repository = parts.employee_family_repository.clone();
        let employee_tax_repository = parts.employee_tax_repository.clone();
        let record_change_service = std::sync::Arc::new(
            crate::application::service::record_change_service::RecordChangeService::new(
                db_pool.clone(),
            ),
        );
        Ok(ModuleExt {
            employee_family_repository: employee_family_repository.clone(),
            employee_tax_repository: employee_tax_repository.clone(),
            db_pool,
            record_change_service,
        })
    }
}

impl crate::ThisModule {
    /// The record-change lifecycle service (for the composing service to
    /// wire its approvals adapter onto).
    pub fn record_change_service(
        &self,
    ) -> std::sync::Arc<crate::application::service::record_change_service::RecordChangeService> {
        self.record_change_service.clone()
    }
    /// The record-change verb router (mount under the host's guarded tree).
    pub fn record_change_verb_routes(&self) -> axum::Router {
        crate::presentation::http::guarded_routes::record_change_verb_routes(
            self.record_change_service.clone(),
        )
    }
}
