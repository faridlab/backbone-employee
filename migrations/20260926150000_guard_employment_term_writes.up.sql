-- Guard: employment_status and end_join_date become single-writer columns.
--
-- The contract record is the load-bearing truth for the employment's term
-- (council 2026-09-26, guard b); the two columns are its projection. A bare
-- PATCH diverging the projection from the contract rows is refused unless the
-- write rides an allowlisted lane: the record-change apply or the probation
-- confirmation, both of which set 'app.allow_employment_term_write' inside
-- their own transaction (the GUC is tx-local, so it can never leak).

CREATE OR REPLACE FUNCTION employee.guard_employment_term_write() RETURNS trigger AS $$
BEGIN
    IF (NEW.employment_status IS DISTINCT FROM OLD.employment_status)
       OR (NEW.end_join_date IS DISTINCT FROM OLD.end_join_date) THEN
        IF COALESCE(current_setting('app.allow_employment_term_write', true), '') <> '1' THEN
            RAISE EXCEPTION 'employment_term_write_refused: employment_status and end_join_date change only through the record-change lane or the probation confirmation (the contract record is the load-bearing truth)';
        END IF;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS employments_term_write_guard ON employee.employments;
CREATE TRIGGER employments_term_write_guard
    BEFORE UPDATE ON employee.employments
    FOR EACH ROW EXECUTE FUNCTION employee.guard_employment_term_write();
