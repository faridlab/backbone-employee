//! The extension of `types.rs` (hand-written; ADR-0031).

#[allow(unused_imports)]
use super::*;

// Add custom public types here

/// The Indonesia statutory inputs payroll needs to compute net pay (PPh 21, BPJS Kesehatan,
/// BPJS Ketenagakerjaan, THR) for one employee. Read via [`super::super::services::EmployeeQueryService::statutory_inputs`].
///
/// `base_salary` is intentionally NOT here: the employee module stores no queryable monthly salary
/// (`EmployeeTax.tax_salary` is a Taxable/NonTaxable flag, not an amount, and there is no
/// CompensationChange entity yet). Payroll supplies the structure's monthly gross as the statutory
/// base when it calls `compute_statutory` — that is the salary actually being paid this run.
///
/// `join_date` is optional because an employee may legally exist without an employment row yet
/// (newly onboarded); a `None` means "tenure unknown" and payroll's THR calc yields zero (no THR).
#[derive(Debug, Clone)]
pub struct StatutoryInputs {
    /// PTKP tier (override wins, else derived from dependents) — drives PPh 21 relief.
    pub ptkp: PtkpTier,
    /// Whether the employee has an NPWP (non-empty `npwp_number`). Drives the 20% PPh 21 surtax
    /// when false.
    pub has_npwp: bool,
    /// BPJS Kesehatan registered family size (`bpjs_kesehatan_family`). Currently informational —
    /// the employee share is 1% of capped salary regardless of family size; the employer share
    /// scales with it (not modeled in the employee-side deduction).
    pub bpjs_kesehatan_family: Option<i32>,
    /// Employment join date — the anchor for THR tenure pro-rating. `None` when no employment row.
    pub join_date: Option<NaiveDate>,
    /// PPh-21 average-effective-rate (TER) category — which TER rate table the monthly withholding
    /// dispatches to. `None` = the progressive-bracket path. Independent of `TaxMethod` on the tax
    /// row (that field describes gross-up treatment, not the withholding rate table).
    pub ter_category: Option<TerCategory>,
}
