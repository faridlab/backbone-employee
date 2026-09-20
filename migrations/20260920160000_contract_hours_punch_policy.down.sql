ALTER TABLE employee.employments
    DROP COLUMN IF EXISTS punch_required,
    DROP COLUMN IF EXISTS contracted_hours_per_week;
