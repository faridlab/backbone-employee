-- Down: drop employee.employee_educations table
DROP TABLE IF EXISTS employee.employee_educations CASCADE;
DROP FUNCTION IF EXISTS employee.employee_educations_audit_timestamp() CASCADE;
