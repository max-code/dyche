-- Add migration script here
CREATE OR REPLACE VIEW player_opponents AS
WITH combined_player_fixtures AS (
    SELECT
        ph.player_id,
        ph.fixture_id,
        ph.was_home AS is_home
    FROM player_history ph
    JOIN fixtures f ON ph.fixture_id = f.id AND f.started = true
    UNION ALL
    SELECT
        pf.player_id,
        pf.fixture_id,
        pf.is_home
    FROM player_fixtures pf
    JOIN fixtures f ON pf.fixture_id = f.id AND f.started = false
)
SELECT 
    p.id AS player_id,
    p.web_name,
    f.game_week_id,
    STRING_AGG(
        CASE 
            WHEN cpf.is_home THEN opp.short_name || ' (H)'
            ELSE opp.short_name || ' (A)'
        END,
        ', ' ORDER BY f.kickoff_time
    ) AS opponents
FROM 
    players p
    JOIN combined_player_fixtures cpf ON p.id = cpf.player_id
    JOIN fixtures f ON f.id = cpf.fixture_id
    JOIN clubs opp ON CASE 
        WHEN cpf.is_home THEN opp.id = f.away_team_id
        ELSE opp.id = f.home_team_id
    END
GROUP BY 
    p.id, p.web_name, f.game_week_id
ORDER BY 
    p.web_name;