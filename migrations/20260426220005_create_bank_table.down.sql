-- Down: drop employee.banks table
DROP TABLE IF EXISTS employee.banks CASCADE;
DROP FUNCTION IF EXISTS employee.banks_audit_timestamp() CASCADE;
