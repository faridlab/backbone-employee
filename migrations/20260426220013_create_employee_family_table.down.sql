-- Down: drop employee.employee_families table
DROP TABLE IF EXISTS employee.employee_families CASCADE;
DROP FUNCTION IF EXISTS employee.employee_families_audit_timestamp() CASCADE;
