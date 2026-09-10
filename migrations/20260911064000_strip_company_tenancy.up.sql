-- Hand-authored (user-owned). Not regenerated.
--
-- Strip every company-fence artifact from the employee tables (ADR-0029): the module is
-- tenant-agnostic; org scoping is installed by the COMPOSING service's tenancy decorator,
-- never by the module. Dropped here, per table: the company-leading indexes, the
-- <table>_company_isolation RLS policy, and the company_id column itself. The
-- employees_public view is rebuilt without its company_id projection because Postgres
-- refuses to drop a column a view depends on.
--
-- Ordering guard (the decorator must run FIRST on any database with data): the module
-- never moves tenancy data. A table is safe to strip when EITHER
--   a) it carries org_unit_id with no NULLs — the decorator backfilled it from company_id —
--      or b) it is empty (a fresh database: the earlier chain files created it empty).
-- Otherwise the strip RAISEs, naming the decorator step, rather than dropping a column
-- that still holds the only tenancy key. The file is re-runnable (every drop is IF EXISTS
-- and the tracker has no checksums), so a failed run retries cleanly after the decorator
-- lands.
--
-- RLS enable/force flags are deliberately NOT touched: the decorator owns those now.
-- The per-unit unique on (org_unit_id, employee_number) that replaces
-- idx_employees_company_id_employee_number is likewise the decorator's posture and is
-- intentionally NOT created here.

DO $$
DECLARE
    t text;
    has_org boolean;
    org_nulls bigint;
    total bigint;
    offenders text := '';
BEGIN
    FOREACH t IN ARRAY ARRAY['employees', 'employments', 'employment_histories', 'employee_identities', 'employee_families', 'employee_contacts', 'employee_educations', 'employee_certifications', 'employee_work_experiences', 'employee_bank_accounts', 'employee_taxes', 'employee_bpjs', 'data_consents', 'data_subject_requests', 'pii_access_logs']
    LOOP
        IF to_regclass(format('employee.%I', t)) IS NULL THEN
            CONTINUE; -- chain not fully applied on this database; nothing to strip
        END IF;

        SELECT EXISTS (
                   SELECT 1 FROM information_schema.columns
                   WHERE table_schema = 'employee' AND table_name = t AND column_name = 'org_unit_id'
               )
        INTO has_org;

        EXECUTE format('SELECT count(*) FROM employee.%I', t) INTO total;

        IF has_org THEN
            EXECUTE format(
                'SELECT count(*) FROM employee.%I WHERE org_unit_id IS NULL', t)
            INTO org_nulls;
        ELSE
            org_nulls := total; -- no org column: every row's only tenancy key is company_id
        END IF;

        IF has_org AND org_nulls = 0 THEN
            CONTINUE; -- decorator backfilled: safe
        END IF;
        IF total = 0 THEN
            CONTINUE; -- empty table (fresh database): safe
        END IF;
        offenders := offenders || format(' employee.%s (%s rows, %s rows not covered by org_unit_id);', t, total, org_nulls);
    END LOOP;

    IF offenders <> '' THEN
        RAISE EXCEPTION 'refusing to strip company_id — these tables are not yet covered by the tenancy decorator:%. Apply the composing service''s tenancy decorator (it backfills org_unit_id from company_id) and re-run; it is the only step that moves tenancy data.', offenders;
    END IF;
END $$;

-- The view projects employees.company_id, so it must go before the column does.
DROP VIEW IF EXISTS employee.employees_public;

-- ── employees ──────────────────────────────────────────────────────────────────
DROP INDEX IF EXISTS employee.idx_employees_company_id;
DROP INDEX IF EXISTS employee.idx_employees_company_id_employee_number;
DROP POLICY IF EXISTS employees_company_isolation ON employee.employees;
ALTER TABLE employee.employees DROP COLUMN IF EXISTS company_id;

-- ── employments ────────────────────────────────────────────────────────────────
DROP INDEX IF EXISTS employee.idx_employments_company_id_status;
DROP POLICY IF EXISTS employments_company_isolation ON employee.employments;
ALTER TABLE employee.employments DROP COLUMN IF EXISTS company_id;

-- ── employment_histories ───────────────────────────────────────────────────────
DROP POLICY IF EXISTS employment_histories_company_isolation ON employee.employment_histories;
ALTER TABLE employee.employment_histories DROP COLUMN IF EXISTS company_id;

-- ── employee_identities ────────────────────────────────────────────────────────
DROP POLICY IF EXISTS employee_identities_company_isolation ON employee.employee_identities;
ALTER TABLE employee.employee_identities DROP COLUMN IF EXISTS company_id;

-- ── employee_families ──────────────────────────────────────────────────────────
DROP POLICY IF EXISTS employee_families_company_isolation ON employee.employee_families;
ALTER TABLE employee.employee_families DROP COLUMN IF EXISTS company_id;

-- ── employee_contacts ──────────────────────────────────────────────────────────
DROP POLICY IF EXISTS employee_contacts_company_isolation ON employee.employee_contacts;
ALTER TABLE employee.employee_contacts DROP COLUMN IF EXISTS company_id;

-- ── employee_educations ────────────────────────────────────────────────────────
DROP POLICY IF EXISTS employee_educations_company_isolation ON employee.employee_educations;
ALTER TABLE employee.employee_educations DROP COLUMN IF EXISTS company_id;

-- ── employee_certifications ────────────────────────────────────────────────────
DROP POLICY IF EXISTS employee_certifications_company_isolation ON employee.employee_certifications;
ALTER TABLE employee.employee_certifications DROP COLUMN IF EXISTS company_id;

-- ── employee_work_experiences ──────────────────────────────────────────────────
DROP POLICY IF EXISTS employee_work_experiences_company_isolation ON employee.employee_work_experiences;
ALTER TABLE employee.employee_work_experiences DROP COLUMN IF EXISTS company_id;

-- ── employee_bank_accounts ─────────────────────────────────────────────────────
DROP POLICY IF EXISTS employee_bank_accounts_company_isolation ON employee.employee_bank_accounts;
ALTER TABLE employee.employee_bank_accounts DROP COLUMN IF EXISTS company_id;

-- ── employee_taxes ─────────────────────────────────────────────────────────────
DROP POLICY IF EXISTS employee_taxes_company_isolation ON employee.employee_taxes;
ALTER TABLE employee.employee_taxes DROP COLUMN IF EXISTS company_id;

-- ── employee_bpjs ──────────────────────────────────────────────────────────────
DROP POLICY IF EXISTS employee_bpjs_company_isolation ON employee.employee_bpjs;
ALTER TABLE employee.employee_bpjs DROP COLUMN IF EXISTS company_id;

-- ── data_consents ──────────────────────────────────────────────────────────────
DROP POLICY IF EXISTS data_consents_company_isolation ON employee.data_consents;
ALTER TABLE employee.data_consents DROP COLUMN IF EXISTS company_id;

-- ── data_subject_requests ──────────────────────────────────────────────────────
DROP INDEX IF EXISTS employee.idx_data_subject_requests_company_id_status;
DROP POLICY IF EXISTS data_subject_requests_company_isolation ON employee.data_subject_requests;
ALTER TABLE employee.data_subject_requests DROP COLUMN IF EXISTS company_id;

-- ── pii_access_logs ────────────────────────────────────────────────────────────
DROP POLICY IF EXISTS pii_access_logs_company_isolation ON employee.pii_access_logs;
ALTER TABLE employee.pii_access_logs DROP COLUMN IF EXISTS company_id;

-- ── Rebuild the peer-visible directory view without the tenancy column ─────────
-- Same shape as the original employees_public view, minus e.company_id.
-- `security_invoker = on` (PG15+) stays load-bearing: the view executes with the
-- CALLER's privileges, so whatever RLS fence the composing service's tenancy
-- decorator installed on employees/employments applies through the view. With the
-- default (definer) semantics the view would run as its owner and silently bypass
-- that fence for every app-role query.
-- On an undecorated deployment neither table carries a policy, so non-superuser
-- roles (FORCE RLS is on) see zero rows through the view until the decorator lands —
-- fail-closed, by design.
CREATE VIEW employee.employees_public WITH (security_invoker = on) AS
SELECT
    e.id,
    e.employee_number,
    e.first_name,
    e.last_name,
    e.email,
    e.mobile_phone,
    e.phone,
    m.department_id,
    m.level_id,
    m.position_id,
    m.direct_manager_id,
    m.employment_status,
    m.join_date,
    m.end_join_date
FROM employee.employees e
LEFT JOIN LATERAL (
    SELECT
        emp.department_id,
        emp.level_id,
        emp.position_id,
        emp.direct_manager_id,
        emp.employment_status,
        emp.join_date,
        emp.end_join_date
    FROM employee.employments emp
    WHERE emp.employee_id = e.id
      AND emp.status = 'active'
      AND (emp.metadata->>'deleted_at') IS NULL
    ORDER BY emp.join_date DESC
    LIMIT 1
) m ON true
WHERE (e.metadata->>'deleted_at') IS NULL;
