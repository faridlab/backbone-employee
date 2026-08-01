-- Down: drop employee.employee_certifications table
DROP TABLE IF EXISTS employee.employee_certifications CASCADE;
DROP FUNCTION IF EXISTS employee.employee_certifications_audit_timestamp() CASCADE;
