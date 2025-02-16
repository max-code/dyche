-- Add migration script here
CREATE OR REPLACE VIEW player_bonus_points AS
WITH player_match_bps AS (
	select 
	    gwp.player_id, 
	    gwp.bps, 
	    f.id as fixture_id,
	    RANK() OVER (PARTITION BY f.id ORDER BY gwp.bps DESC) as bps_rank 
	FROM game_week_players gwp 
	JOIN players p ON p.id = gwp.player_id 
	JOIN clubs c ON p.team = c.id 
	JOIN fixtures f ON 
	    (f.away_team_id = c.id OR f.home_team_id = c.id)
	    AND f.game_week_id = gwp.game_week_id
	where gwp.game_week_id = (select id from current_game_week cgw)
),
bonus_points_calc AS (
  SELECT 
    player_id,
    fixture_id,
    bps,
    CASE 
      WHEN bps_rank = 1 THEN 3
      WHEN bps_rank = 2 THEN 2  
      WHEN bps_rank = 3 THEN 1
      ELSE 0
    END as bonus_points
  FROM player_match_bps
)
SELECT 
  player_id,
  fixture_id,
  bps,
  bonus_points
FROM bonus_points_calc;


