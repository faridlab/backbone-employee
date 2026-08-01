-- Down: drop employee.employee_contacts table
DROP TABLE IF EXISTS employee.employee_contacts CASCADE;
DROP FUNCTION IF EXISTS employee.employee_contacts_audit_timestamp() CASCADE;
