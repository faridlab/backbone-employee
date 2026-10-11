// This directory's extension (hand-written; ADR-0031). The generated mod.rs (lib.rs for
// src/) pulls it in with include!, so its lines resolve against this directory.

pub mod record_change_approvals_port;
pub mod record_change_service;
// Hand-written `EmployeeQueryService` impl (user-owned file). Standard lookups delegate to the
// GenericCrudService aliases above; the `employee_ptkp` read-port delegates to the family/tax repos'
// SQL (override > derived-from-dependents).
pub mod employee_query_service;
// ADR-005 consumer: applies `recruitment.hired` by creating Employee + Employment, idempotently
// (inbox dedup). Registered on the integration bus in backbone-hr-app's main.rs.
pub mod recruitment_hired_handler;
// The employee number a hire receives: the next in the company's sequence, in a settings-driven format.
pub mod employee_numbering;
// ADR-005 consumers: the three career-lifecycle compound events. Each applies its effect idempotently
// (inbox dedup on the envelope id). Registered on the integration bus in backbone-hr-app's main.rs.
pub mod promotion_effective_handler;
pub mod onboarding_completed_handler;
pub mod offboarding_closed_handler;
// ADR-005 consumer: applies `lifecycle.probation_confirmed` by appending a confirmation history
// row and CAS-flipping employment_status probation→permanent, idempotently (inbox dedup).
pub mod probation_confirmed_handler;
// UU PDP consent enforcement (coherence council #2): gates PII writes on valid DataConsent.
pub mod consent_guard;
// UU PDP consent-gated PII writes (Identity/Tax/Bpjs/Family/BankAccount/Contact).
pub mod pii_write_service;
pub use recruitment_hired_handler::{
    hire_first_day, hired_employee_id, hired_employee_id_for, RecruitmentHiredHandler,
};
pub use employee_numbering::{
    allocate_employee_number, DefaultEmployeeNumberFormat, EmployeeNumberFormat,
    EmployeeNumberFormatSource,
};
pub use promotion_effective_handler::PromotionEffectiveHandler;
pub use onboarding_completed_handler::OnboardingCompletedHandler;
pub use offboarding_closed_handler::OffboardingClosedHandler;
pub use probation_confirmed_handler::ProbationConfirmedHandler;
