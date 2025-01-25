-- Add migration script here
ALTER TABLE fixtures
ALTER COLUMN game_week_id TYPE smallint,
ALTER COLUMN home_team_id TYPE smallint,
ALTER COLUMN away_team_id TYPE smallint;
