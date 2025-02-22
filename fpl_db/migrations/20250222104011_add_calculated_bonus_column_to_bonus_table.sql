-- Add migration script here
CREATE VIEW bonus_with_calculated AS
SELECT 
    fixture_id,
    player_id,
    bps,
    bonus,
    CASE 
        WHEN (RANK() OVER (PARTITION BY fixture_id ORDER BY bps DESC)) = 1 THEN 3
        WHEN (RANK() OVER (PARTITION BY fixture_id ORDER BY bps DESC)) = 2 THEN 2
        WHEN (RANK() OVER (PARTITION BY fixture_id ORDER BY bps DESC)) = 3 THEN 1
        ELSE 0
    END AS calculated_bonus
FROM bonus;