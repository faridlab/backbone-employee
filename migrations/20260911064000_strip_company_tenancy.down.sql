-- Hand-authored (user-owned). Not regenerated.
--
-- Best-effort restore sketch for the tenancy strip (ADR-0029). This is a breaking module
-- release against dev-stage databases: the down re-adds the company_id column as nullable
-- with the company-leading indexes and the company isolation policy shape, and rebuilds
-- the employees_public view with its company_id projection — but restores NO data — rows
-- written after the strip (or after the decorator re-keyed them) carry org_unit_id only.
-- The composing service's tenancy decorator remains the live fence; treat this down as a
-- schema-shape sketch for archaeology, not a usable rollback.

ALTER TABLE employee.employees                 ADD COLUMN IF NOT EXISTS company_id uuid;
ALTER TABLE employee.employments               ADD COLUMN IF NOT EXISTS company_id uuid;
ALTER TABLE employee.employment_histories      ADD COLUMN IF NOT EXISTS company_id uuid;
ALTER TABLE employee.employee_identities       ADD COLUMN IF NOT EXISTS company_id uuid;
ALTER TABLE employee.employee_families         ADD COLUMN IF NOT EXISTS company_id uuid;
ALTER TABLE employee.employee_contacts         ADD COLUMN IF NOT EXISTS company_id uuid;
ALTER TABLE employee.employee_educations       ADD COLUMN IF NOT EXISTS company_id uuid;
ALTER TABLE employee.employee_certifications   ADD COLUMN IF NOT EXISTS company_id uuid;
ALTER TABLE employee.employee_work_experiences ADD COLUMN IF NOT EXISTS company_id uuid;
ALTER TABLE employee.employee_bank_accounts    ADD COLUMN IF NOT EXISTS company_id uuid;
ALTER TABLE employee.employee_taxes            ADD COLUMN IF NOT EXISTS company_id uuid;
ALTER TABLE employee.employee_bpjs             ADD COLUMN IF NOT EXISTS company_id uuid;
ALTER TABLE employee.data_consents             ADD COLUMN IF NOT EXISTS company_id uuid;
ALTER TABLE employee.data_subject_requests     ADD COLUMN IF NOT EXISTS company_id uuid;
ALTER TABLE employee.pii_access_logs           ADD COLUMN IF NOT EXISTS company_id uuid;

-- The view projects employees.company_id, so it must go before the column is re-added.
DROP VIEW IF EXISTS employee.employees_public;

CREATE INDEX IF NOT EXISTS idx_employees_company_id
    ON employee.employees (company_id);
CREATE UNIQUE INDEX IF NOT EXISTS idx_employees_company_id_employee_number
    ON employee.employees (company_id, employee_number) WHERE (metadata->>'deleted_at') IS NULL;
CREATE INDEX IF NOT EXISTS idx_employments_company_id_status
    ON employee.employments (company_id, status);
CREATE INDEX IF NOT EXISTS idx_data_subject_requests_company_id_status
    ON employee.data_subject_requests (company_id, status);

CREATE POLICY employees_company_isolation ON employee.employees
    FOR ALL
    USING      (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid)
    WITH CHECK (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid);
CREATE POLICY employments_company_isolation ON employee.employments
    FOR ALL
    USING      (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid)
    WITH CHECK (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid);
CREATE POLICY employment_histories_company_isolation ON employee.employment_histories
    FOR ALL
    USING      (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid)
    WITH CHECK (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid);
CREATE POLICY employee_identities_company_isolation ON employee.employee_identities
    FOR ALL
    USING      (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid)
    WITH CHECK (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid);
CREATE POLICY employee_families_company_isolation ON employee.employee_families
    FOR ALL
    USING      (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid)
    WITH CHECK (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid);
CREATE POLICY employee_contacts_company_isolation ON employee.employee_contacts
    FOR ALL
    USING      (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid)
    WITH CHECK (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid);
CREATE POLICY employee_educations_company_isolation ON employee.employee_educations
    FOR ALL
    USING      (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid)
    WITH CHECK (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid);
CREATE POLICY employee_certifications_company_isolation ON employee.employee_certifications
    FOR ALL
    USING      (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid)
    WITH CHECK (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid);
CREATE POLICY employee_work_experiences_company_isolation ON employee.employee_work_experiences
    FOR ALL
    USING      (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid)
    WITH CHECK (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid);
CREATE POLICY employee_bank_accounts_company_isolation ON employee.employee_bank_accounts
    FOR ALL
    USING      (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid)
    WITH CHECK (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid);
CREATE POLICY employee_taxes_company_isolation ON employee.employee_taxes
    FOR ALL
    USING      (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid)
    WITH CHECK (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid);
CREATE POLICY employee_bpjs_company_isolation ON employee.employee_bpjs
    FOR ALL
    USING      (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid)
    WITH CHECK (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid);
CREATE POLICY data_consents_company_isolation ON employee.data_consents
    FOR ALL
    USING      (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid)
    WITH CHECK (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid);
CREATE POLICY data_subject_requests_company_isolation ON employee.data_subject_requests
    FOR ALL
    USING      (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid)
    WITH CHECK (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid);
CREATE POLICY pii_access_logs_company_isolation ON employee.pii_access_logs
    FOR ALL
    USING      (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid)
    WITH CHECK (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid);

-- Same shape as the original employees_public view, with e.company_id restored.
CREATE VIEW employee.employees_public WITH (security_invoker = on) AS
SELECT
    e.id,
    e.company_id,
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
