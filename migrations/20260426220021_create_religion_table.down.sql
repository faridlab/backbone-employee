-- Down: drop employee.religions table
DROP TABLE IF EXISTS employee.religions CASCADE;
DROP FUNCTION IF EXISTS employee.religions_audit_timestamp() CASCADE;
