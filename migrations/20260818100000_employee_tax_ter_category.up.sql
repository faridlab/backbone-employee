-- PPh-21 average-effective-rate (TER) category on the tax identity: which TER rate table the
-- monthly withholding dispatches to. Nullable on purpose — NULL keeps the progressive-bracket
-- path, so existing rows need no backfill and no default. Independent of tax_method (that field
-- says how the employer grosses up; this one selects the withholding rate table).
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'ter_category') THEN
        CREATE TYPE ter_category AS ENUM ('ter_a', 'ter_b', 'ter_c');
    END IF;
END
$$;

ALTER TABLE employee.employee_taxes
    ADD COLUMN IF NOT EXISTS ter_category ter_category;
