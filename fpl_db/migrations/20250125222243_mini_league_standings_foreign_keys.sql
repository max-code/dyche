-- Add migration script here
ALTER TABLE mini_league_standings ADD CONSTRAINT fk_team FOREIGN KEY (team_id) REFERENCES teams (id),
ADD CONSTRAINT fk_league FOREIGN KEY (league_id) REFERENCES mini_leagues (id);
