-- Down: drop employee.employee_taxes table
DROP TABLE IF EXISTS employee.employee_taxes CASCADE;
DROP FUNCTION IF EXISTS employee.employee_taxes_audit_timestamp() CASCADE;
