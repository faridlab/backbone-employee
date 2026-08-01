-- Migration: Add nullable base_salary to employee.employees (ADR-005 onboarding enrollment).
--
-- ADR-005's `onboarding.completed` → payroll `OnboardingEnrolledHandler` seeds the joiner's INITIAL
-- `payroll.compensation_changes` row from their starting salary. That salary is captured at the
-- employee master (the recruitment offer's negotiated gross), so the handler reads
-- `employee.employees.base_salary` through a pool-backed port (no Cargo dep on backbone-employee — the
-- graph stays acyclic). NULL means "no starting salary recorded yet" → the handler claims-but-skips.
--
-- Scope note: this is an ADDITIVE nullable column — existing rows default to NULL (claim-but-skip), so
-- nothing breaks. The employee module's query-service boundary (no salary exposed for statutory calc)
-- is unchanged: payroll still owns running salary via `compensation_changes`; this column is the
-- one-time recruitment seed, not the running salary of record.

ALTER TABLE employee.employees
    ADD COLUMN IF NOT EXISTS base_salary NUMERIC(18, 2) CHECK (base_salary IS NULL OR base_salary >= 0);
