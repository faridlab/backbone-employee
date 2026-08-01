-- Down: drop employee.examples table
DROP TABLE IF EXISTS employee.examples CASCADE;
DROP FUNCTION IF EXISTS employee.examples_audit_timestamp() CASCADE;
