-- Add migration script here
CREATE OR REPLACE VIEW live_gameweek_points AS
SELECT 
    t.player_first_name,
    t.player_last_name, 
    t.name,
    tgwp.team_id,
    tgw.points as official_total_points,
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
    tgw.points;
