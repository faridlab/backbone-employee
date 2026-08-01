-- Down: drop employee.employee_bank_accounts table
DROP TABLE IF EXISTS employee.employee_bank_accounts CASCADE;
DROP FUNCTION IF EXISTS employee.employee_bank_accounts_audit_timestamp() CASCADE;
