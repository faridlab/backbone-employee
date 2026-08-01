-- Down: drop employee.employee_work_experiences table
DROP TABLE IF EXISTS employee.employee_work_experiences CASCADE;
DROP FUNCTION IF EXISTS employee.employee_work_experiences_audit_timestamp() CASCADE;
