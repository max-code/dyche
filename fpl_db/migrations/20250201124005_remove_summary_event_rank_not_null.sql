-- Add migration script here
ALTER TABLE teams
ALTER COLUMN summary_event_rank
DROP NOT NULL;
