-- Revert the one-active-employment invariant (Wave 1 P1, H-1).
DROP INDEX IF EXISTS employee.employments_one_active_per_employee;
