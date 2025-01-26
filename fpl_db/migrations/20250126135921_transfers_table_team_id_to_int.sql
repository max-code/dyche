-- Add migration script here
ALTER TABLE transfers
ALTER COLUMN team_id TYPE INTEGER;
