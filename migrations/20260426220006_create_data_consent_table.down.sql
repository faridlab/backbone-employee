-- Down: drop employee.data_consents table
DROP TABLE IF EXISTS employee.data_consents CASCADE;
DROP FUNCTION IF EXISTS employee.data_consents_audit_timestamp() CASCADE;
