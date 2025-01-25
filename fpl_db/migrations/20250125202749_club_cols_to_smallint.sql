-- Add migration script here
ALTER TABLE clubs
ALTER COLUMN id TYPE smallint,
ALTER COLUMN strength_overall_home TYPE smallint,
ALTER COLUMN strength_overall_away TYPE smallint,
ALTER COLUMN strength_attack_home TYPE smallint,
ALTER COLUMN strength_attack_away TYPE smallint,
ALTER COLUMN strength_defence_home TYPE smallint,
ALTER COLUMN strength_defence_away TYPE smallint;
