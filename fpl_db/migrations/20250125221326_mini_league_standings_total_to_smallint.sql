-- Add migration script here
ALTER TABLE mini_league_standings
ALTER COLUMN total TYPE SMALLINT;
