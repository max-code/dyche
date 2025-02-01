-- Add migration script here
ALTER TABLE team_game_weeks
DROP CONSTRAINT team_game_weeks_team_id_fkey;
