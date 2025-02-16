-- Add migration script here
-- First create the materialized view
CREATE MATERIALIZED VIEW live_gameweek_points AS
SELECT 
    t.player_first_name,
    t.player_last_name, 
    t.name,
    tgwp.team_id,
    tgw.points as official_points,
    SUM(gwp.total_points) as calculated_total_points
FROM team_game_week_picks tgwp
JOIN game_week_players gwp 
    ON tgwp.player_id = gwp.player_id 
    AND tgwp.game_week_id = gwp.game_week_id
JOIN teams t 
    ON t.id = tgwp.team_id
JOIN team_game_weeks tgw 
    ON tgw.team_id = tgwp.team_id 
    AND tgw.game_week_id = tgwp.game_week_id
WHERE tgwp.game_week_id = 25
GROUP BY 
    t.player_first_name,
    t.player_last_name,
    t.name,
    tgwp.team_id,
    tgw.points
WITH DATA;

-- Create indexes to improve query performance
CREATE UNIQUE INDEX live_gameweek_points_team_id_idx ON live_gameweek_points(team_id);
CREATE INDEX live_gameweek_points_calculated_points_idx ON live_gameweek_points(calculated_total_points);
CREATE INDEX live_gameweek_points_official_points_idx ON live_gameweek_points(official_points);

-- Create a function to refresh the materialized view
CREATE OR REPLACE FUNCTION refresh_live_gameweek_points()
RETURNS TRIGGER AS $$
BEGIN
    REFRESH MATERIALIZED VIEW CONCURRENTLY live_gameweek_points;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Create triggers to refresh the view when underlying tables change
CREATE TRIGGER refresh_live_gameweek_points_team_game_week_picks
AFTER INSERT OR UPDATE OR DELETE ON team_game_week_picks
FOR EACH STATEMENT EXECUTE FUNCTION refresh_live_gameweek_points();

CREATE TRIGGER refresh_live_gameweek_points_game_week_players
AFTER INSERT OR UPDATE OR DELETE ON game_week_players
FOR EACH STATEMENT EXECUTE FUNCTION refresh_live_gameweek_points();

CREATE TRIGGER refresh_live_gameweek_points_teams
AFTER INSERT OR UPDATE OR DELETE ON teams
FOR EACH STATEMENT EXECUTE FUNCTION refresh_live_gameweek_points();

CREATE TRIGGER refresh_live_gameweek_points_team_game_weeks
AFTER INSERT OR UPDATE OR DELETE ON team_game_weeks
FOR EACH STATEMENT EXECUTE FUNCTION refresh_live_gameweek_points();