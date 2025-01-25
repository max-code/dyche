-- Add migration script here
CREATE TABLE IF NOT EXISTS mini_leagues (
    id INTEGER PRIMARY KEY NOT NULL,
    last_updated_data TIMESTAMPTZ NOT NULL,
    name TEXT NOT NULL,
    created TIMESTAMPTZ NOT NULL,
    closed BOOL NOT NULL,
    max_entries INTEGER,
    league_type TEXT NOT NULL,
    scoring TEXT NOT NULL,
    admin_entry SMALLINT NOT NULL,
    start_event SMALLINT NOT NULL,
    code_privacy TEXT NOT NULL,
    has_cup BOOL NOT NULL,
    cup_league INTEGER,
    rank INTEGER
)
