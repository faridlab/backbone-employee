-- Down: drop employee.employee_identities table
DROP TABLE IF EXISTS employee.employee_identities CASCADE;
DROP FUNCTION IF EXISTS employee.employee_identities_audit_timestamp() CASCADE;
