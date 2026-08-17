-- Add the 'confirmation' variant to the employment_action enum: a probation
-- that ran to its end (or was confirmed early by an operator) is a real
-- employment milestone and belongs in the history timeline next to hires and
-- transfers, not folded into a note.
--
-- This file is intentionally a single statement: ALTER TYPE ... ADD VALUE
-- must run alone (Postgres restriction on enum-value changes inside a
-- multi-statement transaction). IF NOT EXISTS makes re-runs harmless.

ALTER TYPE employment_action ADD VALUE IF NOT EXISTS 'confirmation';
