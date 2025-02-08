-- Add migration script here
ALTER TABLE fixtures
ALTER COLUMN game_week_id
DROP NOT NULL,
ALTER COLUMN kickoff_time
DROP NOT NULL,
ALTER COLUMN started
DROP NOT NULL;
