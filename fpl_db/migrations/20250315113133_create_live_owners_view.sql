-- Add migration script here

DROP VIEW IF EXISTS live_owners;

CREATE VIEW live_owners AS
SELECT 
    p.web_name,
    gwp.total_points,
    p.id AS player_id,
    STRING_AGG(DISTINCT du.discord_id::text, ',') AS "owners"
FROM 
    players p
JOIN 
    game_week_players gwp ON p.id = gwp.player_id 
JOIN 
    team_game_week_picks tgwp ON p.id = tgwp.player_id AND gwp.game_week_id = tgwp.game_week_id
JOIN 
    discord_users du ON tgwp.team_id = du.team_id
WHERE 
    tgwp.game_week_id = (SELECT id FROM current_game_week)
GROUP BY 
    p.id, p.web_name, gwp.total_points; 