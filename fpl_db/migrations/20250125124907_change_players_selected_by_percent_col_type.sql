-- Add migration script here
ALTER TABLE players
ALTER COLUMN selected_by_percent TYPE REAL USING selected_by_percent::real;
