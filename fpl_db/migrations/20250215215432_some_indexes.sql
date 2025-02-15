-- Add migration script here
CREATE INDEX idx_team_game_week_picks_game_week_player
ON team_game_week_picks (game_week_id, player_id);

CREATE INDEX idx_players_team
ON players (team);
