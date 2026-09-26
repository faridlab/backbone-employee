DROP TRIGGER IF EXISTS employments_term_write_guard ON employee.employments;
DROP FUNCTION IF EXISTS employee.guard_employment_term_write();
