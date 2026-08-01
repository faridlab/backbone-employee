-- Down: drop employee.employee_bpjs table
DROP TABLE IF EXISTS employee.employee_bpjs CASCADE;
DROP FUNCTION IF EXISTS employee.employee_bpjs_audit_timestamp() CASCADE;
