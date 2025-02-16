-- Add migration script here
-- Drop triggers
DROP TRIGGER IF EXISTS refresh_live_gameweek_points_team_game_week_picks ON team_game_week_picks;
DROP TRIGGER IF EXISTS refresh_live_gameweek_points_game_week_players ON game_week_players;
DROP TRIGGER IF EXISTS refresh_live_gameweek_points_teams ON teams;
DROP TRIGGER IF EXISTS refresh_live_gameweek_points_team_game_weeks ON team_game_weeks;

-- Drop function
DROP FUNCTION IF EXISTS refresh_live_gameweek_points();

-- Drop the view itself
DROP MATERIALIZED VIEW IF EXISTS live_gameweek_points;

-- Drop indexes (if they exist)
DROP INDEX IF EXISTS live_gameweek_points_team_id_idx;
DROP INDEX IF EXISTS live_gameweek_points_points_idx;
DROP INDEX IF EXISTS live_gameweek_points_calculated_points_idx;
DROP INDEX IF EXISTS live_gameweek_points_official_points_idx;