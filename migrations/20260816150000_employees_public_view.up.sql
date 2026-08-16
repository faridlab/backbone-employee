-- hr.employee.public port (Wave 1 P1, H-1): the peer-visible employee directory.
--
-- A read-only SQL VIEW over employees + the current (active) employment — the Odoo
-- `hr.employee.public` equivalent, redacted harder than Odoo on purpose: civil-registration
-- and health-adjacent PII (birth place/date, gender, marital status, blood type, religion)
-- are NOT projected at all, while contact fields peers need to reach colleagues (email,
-- mobile, phone) ARE. Full-record reads stay behind the consent guard (UU PDP) + pii_access_log.
--
-- `security_invoker = on` (PG15+) is load-bearing: the view executes with the CALLER's
-- privileges, so the ADR-0014 RLS fence on employees/employments applies through the view.
-- With the default (definer) semantics the view would run as its owner and silently bypass
-- the fence for every app-role query — a cross-company leak, not a convenience.

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
