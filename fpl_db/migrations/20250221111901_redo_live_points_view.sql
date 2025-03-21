-- Add migration script here
DROP VIEW IF EXISTS live_points;

CREATE VIEW live_points AS
WITH current_gw AS (
    SELECT id
    FROM current_game_week cgw
),
bonus_summary AS (
    SELECT b.player_id, SUM(b.bonus) AS total_bonus
    FROM bonus b
    JOIN fixtures f ON f.id = b.fixture_id
    CROSS JOIN current_gw
    WHERE f.game_week_id = current_gw.id
    GROUP BY b.player_id
)
SELECT
    tgw.game_week_id,
    t.player_first_name,
    t.player_last_name,
    t.name,
    tgwp.team_id,
    du.discord_id,
    tgw.points AS week_points,
    SUM(tgwp.multiplier * (gwp.total_points + COALESCE(bs.total_bonus, 0) - gwp.bonus)) AS calculated_week_points,
    t.summary_overall_points AS overall_points,
    (t.summary_overall_points 
     + SUM(tgwp.multiplier * (gwp.total_points + COALESCE(bs.total_bonus, 0) - gwp.bonus))
     - t.summary_event_points) AS calculated_overall_points
FROM team_game_week_picks tgwp
JOIN current_gw ON true
JOIN game_week_players gwp
    ON tgwp.player_id = gwp.player_id
    AND tgwp.game_week_id = gwp.game_week_id
JOIN teams t
    ON t.id = tgwp.team_id
JOIN team_game_weeks tgw
    ON tgw.team_id = tgwp.team_id
    AND tgw.game_week_id = tgwp.game_week_id
LEFT JOIN bonus_summary bs
    ON bs.player_id = tgwp.player_id
LEFT JOIN discord_users du
    ON du.team_id = tgwp.team_id
WHERE tgw.game_week_id = (SELECT id FROM current_gw)
GROUP BY 
    tgw.game_week_id,
    t.player_first_name,
    t.player_last_name,
    t.name,
    tgwp.team_id,
    du.discord_id,
    tgw.points,
    t.summary_overall_points,
    t.summary_event_points
ORDER BY tgw.game_week_id DESC;
