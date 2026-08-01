-- Down: drop employee.data_subject_requests table
DROP TABLE IF EXISTS employee.data_subject_requests CASCADE;
DROP FUNCTION IF EXISTS employee.data_subject_requests_audit_timestamp() CASCADE;
