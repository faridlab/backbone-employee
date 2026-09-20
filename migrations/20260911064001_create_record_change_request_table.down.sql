-- Down: drop employee.record_change_requests table
DROP TABLE IF EXISTS employee.record_change_requests CASCADE;
DROP FUNCTION IF EXISTS employee.record_change_requests_audit_timestamp() CASCADE;
