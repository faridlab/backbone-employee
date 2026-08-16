# The peer directory (`employee.employees_public`) — the `hr.employee.public` port

> Wave 1 P1 / pillar H-1. Spec: metaphora `docs/odoo/human-resources/hr/` (HRM-2, HRF-3,
> D1). landed 2026-08-16.

## What it is

Odoo ships `hr.employee.public` as a **SQL VIEW** (`_auto = False`) every internal user can
browse (`base.group_user` read-only) — the Directory / de-facto org chart. This module's
equivalent is the `employee.employees_public` view + the GET-only handler around it
(`create_employee_public_read_routes`, composed via `create_readonly_employee_routes_with_public`).

## Redaction contract (stricter than Odoo, on purpose)

Odoo's public view projects gender, marital status and ssnid. We do not — the module
already carries a UU PDP consent guard + `pii_access_log` for full-record reads, and a
"public" surface that leaks civil-registration PII would undercut that posture. The view
projects:

| Projected (peers need it to reach/place a colleague) | Excluded (PII — never leaves the full-record path) |
|---|---|
| id, company_id, employee_number | birth_place, birth_date |
| first_name, last_name | gender, marital_status |
| email, mobile_phone, phone | blood_type, religion_id |
| department_id, level_id, position_id, direct_manager_id | user_id |
| employment_status, join_date, end_join_date | |

Redaction is **structural**: the columns are not in the view, so no query against it —
current or future — can select them (`tests/employees_public_test.rs::epp1`). Any new column
on `employees` joins this checklist at review time.

## Fence behavior (the load-bearing detail)

The view is created `WITH (security_invoker = on)` (PG15+). Views default to running with
the **owner's** privileges, which would make every app-role query bypass the ADR-0014 RLS
fence on `employees`/`employments` — a cross-company leak wearing a directory's clothes.
`security_invoker` makes the view execute as the caller, so the company fence applies
*through* the view. `epp5` proves it: as a NOSUPERUSER role, company A sees exactly its own
rows and an unset `app.company_id` sees zero rows (fail-closed).

## Employment semantics — the "stateless, versioned" posture, verified

Odoo keeps `hr.employee` stateless: placement/lifecycle lives on `hr.version` rows, with
`current_version_id` pointing at the effective one. The decomposition here:

| Odoo | here |
|---|---|
| `hr.employee` (stateless shell) | `employees` — no placement/lifecycle columns |
| `hr.version` (versioned placement) | `employments` — "current" = `status = 'active'` |
| `current_version_id` pointer | the one-active-employment **unique index** (below) |
| append-only change log | `employment_histories` (from/to snapshot per action) |

Verified 2026-08-16: the posture holds — `employees` carries only identity, every placement
fact lives on `employments`, and the history log is append-only by write-path convention.

The one structural gap the verification surfaced: "the active employment" was a convention,
not a constraint — two active rows would fork every placement read (and the directory's
LATERAL pick). `20260816150100_employment_one_active` closes it with a partial unique index
(`ON employments (employee_id) WHERE status = 'active' AND metadata->>'deleted_at' IS NULL`)
— DB-level, fires on raw SQL (ADR-0015), and re-verified live by `epp4`.

## Files

| File | Role |
|---|---|
| `migrations/20260816150000_employees_public_view.{up,down}.sql` | the view (security_invoker) |
| `migrations/20260816150100_employment_one_active.{up,down}.sql` | one-active invariant |
| `src/infrastructure/persistence/employee_public_repository.rs` | the only SQL that reads the view |
| `src/presentation/http/employee_public_handler.rs` | GET-only routes |
| `tests/employees_public_test.rs` | EPP-1..5 behavior oracle |
