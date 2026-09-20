-- The contract's bought hours and the person's punch policy: two columns
-- the day/capacity screens rest on.

ALTER TABLE employee.employments
    ADD COLUMN IF NOT EXISTS contracted_hours_per_week numeric(5,2) NOT NULL DEFAULT 40;
ALTER TABLE employee.employments
    ADD COLUMN IF NOT EXISTS punch_required boolean NOT NULL DEFAULT true;
