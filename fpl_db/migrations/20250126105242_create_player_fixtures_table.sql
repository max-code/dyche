-- Add migration script here
CREATE TABLE IF NOT EXISTS player_fixtures (
    fixture_id SMALLINT NOT NULL REFERENCES fixtures (id),
    player_id SMALLINT NOT NULL REFERENCES players (id),
    event_name TEXT NOT NULL,
    is_home BOOL NOT NULL,
    difficulty SMALLINT NOT NULL
)
