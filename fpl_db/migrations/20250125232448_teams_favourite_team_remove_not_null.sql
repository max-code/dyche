-- Add migration script here
ALTER TABLE teams
ALTER COLUMN favourite_team
DROP NOT NULL;
