-- Add migration script here
CREATE OR REPLACE VIEW live_points AS
SELECT 
    t.player_first_name,
    t.player_last_name,
    t.name,
    tgwp.team_id,
    du.discord_id,
    tgw.points AS week_points,
    SUM(tgwp.multiplier * gwp.total_points) AS calculated_week_points,
    t.summary_overall_points AS overall_points,
    t.summary_overall_points + (SUM(tgwp.multiplier * gwp.total_points) - tgw.points) AS calculated_overall_points
FROM team_game_week_picks tgwp
JOIN game_week_players gwp ON tgwp.player_id = gwp.player_id AND tgwp.game_week_id = gwp.game_week_id
JOIN teams t ON t.id = tgwp.team_id
JOIN team_game_weeks tgw ON tgw.team_id = tgwp.team_id AND tgw.game_week_id = tgwp.game_week_id
LEFT JOIN discord_users du ON du.team_id = tgwp.team_id
WHERE tgwp.game_week_id = (select id from current_game_week)
GROUP BY t.player_first_name, t.player_last_name, t.name, tgwp.team_id, du.discord_id, tgw.points, t.summary_overall_points;