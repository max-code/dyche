-- Add migration script here
-- Indexes on disocrd users table
CREATE INDEX IF NOT EXISTS idx_discord_users_discord_id ON discord_users (discord_id);

CREATE INDEX IF NOT EXISTS idx_mini_league_standings_league_team ON mini_league_standings (league_id, team_id);

-- View for user mini leagues
CREATE MATERIALIZED VIEW discord_user_mini_leagues AS
SELECT DISTINCT
    du.discord_id,
    ml.name
FROM
    mini_league_standings mls
    JOIN mini_leagues ml ON ml.id = mls.league_id
    JOIN discord_users du ON du.team_id = mls.team_id;

CREATE INDEX idx_discord_user_mini_leagues_discord_id ON discord_user_mini_leagues (discord_id);

-- Function to refresh the view
CREATE OR REPLACE FUNCTION refresh_discord_user_mini_leagues()
RETURNS TRIGGER AS $$
BEGIN
    REFRESH MATERIALIZED VIEW CONCURRENTLY discord_user_mini_leagues;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Triggers to call the above
CREATE TRIGGER refresh_discord_user_mini_leagues_on_ml_change
    AFTER INSERT OR UPDATE OR DELETE ON mini_leagues
    FOR EACH STATEMENT
    EXECUTE FUNCTION refresh_discord_user_mini_leagues();

CREATE TRIGGER refresh_discord_user_mini_leagues_on_mls_change
    AFTER INSERT OR UPDATE OR DELETE ON mini_league_standings
    FOR EACH STATEMENT
    EXECUTE FUNCTION refresh_discord_user_mini_leagues();

CREATE TRIGGER refresh_discord_user_mini_leagues_on_du_change
    AFTER INSERT OR UPDATE OR DELETE ON discord_users
    FOR EACH STATEMENT
    EXECUTE FUNCTION refresh_discord_user_mini_leagues();
