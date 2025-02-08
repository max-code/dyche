-- Add migration script here
ALTER TABLE player_fixtures
ALTER COLUMN event_name
DROP NOT NULL
