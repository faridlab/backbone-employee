-- Inverse of 20260818100000_employee_tax_ter_category.up.sql. Values are gone with the column;
-- the enum type is kept-dropped last so a re-run of the up migration recreates both cleanly.
ALTER TABLE employee.employee_taxes
    DROP COLUMN IF EXISTS ter_category;

DROP TYPE IF EXISTS ter_category;
