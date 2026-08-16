//! Guarded route composition for the Employee module (user-owned).
//!
//! `EmployeeModule::routes()` (the deprecated alias) and `all_crud_routes()`
//! mount UNVALIDATED generic CRUD on every entity; every service field on the
//! module is `pub(crate)`, so a composing app cannot build a read-only router
//! itself. This file gives it the guarded surface, mirroring the
//! `readonly_routes()` convention on `TimeoffModule`:
//!
//! - [`EmployeeModule::readonly_routes`] — every entity mounted GET-only.
//! - [`EmployeeModule::readonly_routes_with_public`] — the above plus the
//!   `hr.employee.public` PII-redacted peer directory
//!   ([`crate::presentation::http::create_employee_public_read_routes`]).
//!
//! Validated writes (the PII write service's consent-gated surface) merge onto
//! this base when their HTTP layer lands.

use axum::Router;

impl crate::EmployeeModule {
    /// Read-only routes for every employee entity (GET endpoints only) — the
    /// safe base for a guarded composition.
    ///
    /// Generic mutation can't reach here, so this surface cannot bypass the
    /// PII write service's consent invariants. Merge validated write routes
    /// onto it; `all_crud_routes()` remains the explicit unguarded opt-in.
    pub fn readonly_routes(&self) -> Router {
        use crate::presentation::http::{
            create_bank_read_routes, create_data_consent_read_routes,
            create_data_subject_request_read_routes, create_employee_read_routes,
            create_employee_bank_account_read_routes, create_employee_bpjs_read_routes,
            create_employee_certification_read_routes, create_employee_contact_read_routes,
            create_employee_education_read_routes, create_employee_family_read_routes,
            create_employee_identity_read_routes, create_employee_tax_read_routes,
            create_employee_work_experience_read_routes, create_employment_read_routes,
            create_employment_history_read_routes, create_pii_access_log_read_routes,
            create_religion_read_routes,
        };

        Router::new()
            .merge(create_bank_read_routes(self.bank_service.clone()))
            .merge(create_data_consent_read_routes(self.data_consent_service.clone()))
            .merge(create_data_subject_request_read_routes(
                self.data_subject_request_service.clone(),
            ))
            .merge(create_employee_read_routes(self.employee_service.clone()))
            .merge(create_employee_bank_account_read_routes(
                self.employee_bank_account_service.clone(),
            ))
            .merge(create_employee_bpjs_read_routes(self.employee_bpjs_service.clone()))
            .merge(create_employee_certification_read_routes(
                self.employee_certification_service.clone(),
            ))
            .merge(create_employee_contact_read_routes(self.employee_contact_service.clone()))
            .merge(create_employee_education_read_routes(
                self.employee_education_service.clone(),
            ))
            .merge(create_employee_family_read_routes(self.employee_family_service.clone()))
            .merge(create_employee_identity_read_routes(
                self.employee_identity_service.clone(),
            ))
            .merge(create_employee_tax_read_routes(self.employee_tax_service.clone()))
            .merge(create_employee_work_experience_read_routes(
                self.employee_work_experience_service.clone(),
            ))
            .merge(create_employment_read_routes(self.employment_service.clone()))
            .merge(create_employment_history_read_routes(
                self.employment_history_service.clone(),
            ))
            .merge(create_pii_access_log_read_routes(self.pii_access_log_service.clone()))
            .merge(create_religion_read_routes(self.religion_service.clone()))
    }

    /// [`Self::readonly_routes`] plus the `hr.employee.public` peer directory
    /// (Wave 1 P1, H-1) — the guarded composition a consumer mounts.
    pub fn readonly_routes_with_public(&self) -> Router {
        self.readonly_routes().merge(
            crate::presentation::http::create_employee_public_read_routes(self.db_pool.clone()),
        )
    }
}
