-- Add migration script here
ALTER TABLE team_game_weeks ADD CONSTRAINT team_game_weeks_team_id_fkey FOREIGN KEY (team_id) REFERENCES teams (id);
