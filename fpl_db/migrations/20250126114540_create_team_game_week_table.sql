-- Add migration script here
CREATE TABLE IF NOT EXISTS team_game_weeks (
    team_id INTEGER NOT NULL REFERENCES teams (id),
    game_week_id SMALLINT NOT NULL REFERENCES game_weeks (id),
    active_chip TEXT,
    points SMALLINT NOT NULL,
    total_points SMALLINT NOT NULL,
    rank INTEGER NOT NULL,
    rank_sort INTEGER NOT NULL,
    overall_rank INTEGER NOT NULL,
    percentile_rank SMALLINT NOT NULL,
    bank SMALLINT NOT NULL,
    value SMALLINT NOT NULL,
    event_transfers SMALLINT NOT NULL,
    event_transfers_cost SMALLINT NOT NULL,
    points_on_bench SMALLINT NOT NULL,
    PRIMARY KEY (team_id, game_week_id)
);

CREATE TABLE IF NOT EXISTS team_game_week_picks (
    team_id INTEGER NOT NULL REFERENCES teams (id),
    game_week_id SMALLINT NOT NULL REFERENCES game_weeks (id),
    player_id SMALLINT NOT NULL REFERENCES players (id),
    position SMALLINT NOT NULL,
    multiplier SMALLINT NOT NULL,
    is_captain BOOL NOT NULL,
    is_vice_captain BOOL NOT NULL,
    element_type TEXT NOT NULL,
    PRIMARY KEY (team_id, game_week_id, player_id)
);

CREATE TABLE IF NOT EXISTS team_game_week_automatic_subs (
    team_id INTEGER NOT NULL REFERENCES teams (id),
    game_week_id SMALLINT NOT NULL REFERENCES game_weeks (id),
    player_in_id SMALLINT NOT NULL REFERENCES players (id),
    player_out_id SMALLINT NOT NULL REFERENCES players (id),
    PRIMARY KEY (
        team_id,
        game_week_id,
        player_in_id,
        player_out_id
    )
);
