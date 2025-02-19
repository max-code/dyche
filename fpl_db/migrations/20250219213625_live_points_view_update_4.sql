-- Add migration script here
CREATE OR REPLACE VIEW live_points AS
SELECT 
    t.player_first_name,
    t.player_last_name,
    t.name,
    tgwp.team_id,
    du.discord_id,
    tgw.points AS week_points,
    SUM(tgwp.multiplier * (gwp.total_points + 
        CASE 
            WHEN gwp.bonus = 0 THEN COALESCE(pbp.total_bonus_points, 0)
            ELSE 0
        END))::bigint AS calculated_week_points,
    t.summary_overall_points AS overall_points,
    (t.summary_overall_points + (SUM(tgwp.multiplier * (gwp.total_points + 
        CASE 
            WHEN gwp.bonus = 0 THEN COALESCE(pbp.total_bonus_points, 0)
            ELSE 0
        END)) - tgw.points))::bigint AS calculated_overall_points
FROM team_game_week_picks tgwp
JOIN game_week_players gwp ON tgwp.player_id = gwp.player_id AND tgwp.game_week_id = gwp.game_week_id
JOIN teams t ON t.id = tgwp.team_id
JOIN team_game_weeks tgw ON tgw.team_id = tgwp.team_id AND tgw.game_week_id = tgwp.game_week_id
LEFT JOIN (
    SELECT player_id, SUM(bonus_points) as total_bonus_points
    FROM player_bonus_points
    GROUP BY player_id
) pbp ON tgwp.player_id = pbp.player_id
LEFT JOIN discord_users du ON du.team_id = tgwp.team_id
WHERE tgwp.game_week_id = (SELECT current_game_week.id FROM current_game_week)
GROUP BY t.player_first_name, t.player_last_name, t.name, tgwp.team_id, du.discord_id, tgw.points, t.summary_overall_points;