-- Migration: employees gain the recruitment candidate link
--
-- The hire hand-off records which candidate the employee was hired from
-- (carried by the recruitment.hired event since recruitment v0.3.17): the
-- one-person-one-row invariant spans the funnel and the people master.

ALTER TABLE employee.employees
    ADD COLUMN IF NOT EXISTS candidate_id uuid;
