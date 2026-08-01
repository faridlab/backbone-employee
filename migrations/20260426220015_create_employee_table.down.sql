-- Down: drop employee.employees table
DROP TABLE IF EXISTS employee.employees CASCADE;
DROP FUNCTION IF EXISTS employee.employees_audit_timestamp() CASCADE;
