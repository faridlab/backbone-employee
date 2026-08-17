//! Guarded route composition for the Employee module (user-owned).
//!
//! `EmployeeModule::routes()` (the deprecated alias) and `all_crud_routes()`
//! mount UNVALIDATED generic CRUD on every entity; the base read-only router
//! comes from the generated [`EmployeeModule::readonly_routes`]. This file
//! adds what the generator cannot know:
//!
//! - [`EmployeeModule::readonly_routes_with_public`] — the read-only base plus
//!   the `hr.employee.public` PII-redacted peer directory
//!   ([`crate::presentation::http::create_employee_public_read_routes`]).
//!
//! Validated writes (the PII write service's consent-gated surface) merge onto
//! this base when their HTTP layer lands.

use axum::Router;

impl crate::EmployeeModule {
    /// [`Self::readonly_routes`] plus the `hr.employee.public` peer directory —
    /// the guarded composition a consumer mounts.
    pub fn readonly_routes_with_public(&self) -> Router {
        self.readonly_routes().merge(
            crate::presentation::http::create_employee_public_read_routes(self.db_pool.clone()),
        )
    }
}
