-- Reverting an enum-value addition is not supported by Postgres (values
-- cannot be dropped once added). This down-migration is therefore a
-- deliberate no-op: rolling back this migration removes it from the ledger
-- but leaves the 'confirmation' variant in place — harmless because the
-- variant is additive and no code path depends on its absence.

SELECT 1;
