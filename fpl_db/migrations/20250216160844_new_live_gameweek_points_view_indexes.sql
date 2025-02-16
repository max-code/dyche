-- Add migration script here
-- Index for team_game_week_picks table - covers the WHERE clause and JOIN
CREATE INDEX IF NOT EXISTS idx_tgwp_gw_team_player 
ON team_game_week_picks(game_week_id, team_id, player_id);

-- Index for game_week_players table - covers the JOIN condition
CREATE INDEX IF NOT EXISTS idx_gwp_gw_player 
ON game_week_players(game_week_id, player_id, total_points);

-- Index for team_game_weeks table - covers the JOIN conditions
CREATE INDEX IF NOT EXISTS idx_tgw_team_gw_points 
ON team_game_weeks(team_id, game_week_id, points);

-- Index for teams table - covers the grouped columns
CREATE INDEX IF NOT EXISTS idx_teams_summary 
ON teams(id, player_first_name, player_last_name, name, summary_overall_points);