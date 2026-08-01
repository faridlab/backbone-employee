-- Down migration: drop base_salary from employee.employees (ADR-005 onboarding enrollment).

ALTER TABLE employee.employees
    DROP COLUMN IF EXISTS base_salary;
