-- Add migration script here
CREATE TABLE IF NOT EXISTS mini_league_standings (
    id INTEGER NOT NULL,
    event_total SMALLINT NOT NULL,
    player_name TEXT NOT NULL,
    rank INTEGER NOT NULL,
    last_rank INTEGER NOT NULL,
    rank_sort INTEGER NOT NULL,
    total INTEGER,
    team_id INTEGER NOT NULL,
    entry_name TEXT NOT NULL,
    has_player BOOL NOT NULL,
    league_id INTEGER NOT NULL,
    PRIMARY KEY (league_id, team_id)
)
