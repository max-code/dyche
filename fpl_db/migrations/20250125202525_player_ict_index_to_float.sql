-- Add migration script here
-- Add migration script here
ALTER TABLE players
ALTER COLUMN ict_index TYPE REAL USING ict_index::real
