-- Down: drop employee.employments table
DROP TABLE IF EXISTS employee.employments CASCADE;
DROP FUNCTION IF EXISTS employee.employments_audit_timestamp() CASCADE;
