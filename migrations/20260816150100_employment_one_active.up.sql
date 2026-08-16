-- Stateless-versioned employment semantics, made structural (Wave 1 P1, H-1).
--
-- Odoo's `hr.employee` is stateless: placement/lifecycle lives on `hr.version` rows and
-- `current_version_id` points at the effective one. Our equivalent decomposition:
--   employees            = the stateless identity row (no placement/lifecycle columns)
--   employments          = the version rows ("current" = status 'active')
--   employment_histories = the append-only change log (from/to snapshot per action)
-- The pointer equivalent of `current_version_id` is "the active employment" — until now
-- that was a convention, not a constraint: two active employments would silently fork the
-- employees_public projection and every placement read.
--
-- This partial unique index makes "at most one active employment per employee" a DB-level
-- invariant (ADR-0015 default: invariants live in the DB, firing on raw SQL too). Soft
-- delete lives in the metadata JSONB, so the predicate mirrors the module convention
-- (`deleted_at` key inside `metadata`, not a column).

CREATE UNIQUE INDEX IF NOT EXISTS employments_one_active_per_employee
    ON employee.employments (employee_id)
    WHERE status = 'active'
      AND (metadata->>'deleted_at') IS NULL;
