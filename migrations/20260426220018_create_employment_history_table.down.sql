-- Down: drop employee.employment_histories table
DROP TABLE IF EXISTS employee.employment_histories CASCADE;
DROP FUNCTION IF EXISTS employee.employment_histories_audit_timestamp() CASCADE;
