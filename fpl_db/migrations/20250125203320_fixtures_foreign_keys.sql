-- Add migration script here
ALTER TABLE fixtures ADD CONSTRAINT fk_game_week FOREIGN KEY (game_week_id) REFERENCES game_weeks (id);

ALTER TABLE fixtures ADD CONSTRAINT fk_home_team FOREIGN KEY (home_team_id) REFERENCES clubs (id);

ALTER TABLE fixtures ADD CONSTRAINT fk_away_team FOREIGN KEY (away_team_id) REFERENCES clubs (id);
