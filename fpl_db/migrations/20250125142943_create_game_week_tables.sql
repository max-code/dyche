-- Add migration script here
CREATE TABLE IF NOT EXISTS game_weeks (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    deadline_time TIMESTAMPTZ NOT NULL,
    release_time TIMESTAMPTZ,
    average_entry_score SMALLINT NOT NULL,
    finished BOOLEAN NOT NULL,
    data_checked BOOLEAN NOT NULL,
    highest_scoring_entry INTEGER,
    deadline_time_epoch BIGINT NOT NULL,
    deadline_time_game_offset INTEGER NOT NULL,
    highest_score SMALLINT,
    is_previous BOOLEAN NOT NULL,
    is_current BOOLEAN NOT NULL,
    is_next BOOLEAN NOT NULL,
    cup_leagues_created BOOLEAN NOT NULL,
    h2h_ko_matches_created BOOLEAN NOT NULL,
    can_enter BOOLEAN NOT NULL,
    can_manage BOOLEAN NOT NULL,
    released BOOLEAN NOT NULL,
    ranked_count INTEGER NOT NULL,
    transfers_made INTEGER NOT NULL,
    most_selected INTEGER REFERENCES players (id),
    most_transferred_in INTEGER REFERENCES players (id),
    top_element INTEGER REFERENCES players (id),
    most_captained INTEGER REFERENCES players (id),
    most_vice_captained INTEGER REFERENCES players (id)
);

CREATE TABLE IF NOT EXISTS game_week_chip_plays (
    game_week_id INTEGER NOT NULL REFERENCES game_weeks (id),
    chip_name TEXT NOT NULL,
    num_played INTEGER NOT NULL,
    PRIMARY KEY (game_week_id, chip_name)
);

CREATE TABLE IF NOT EXISTS game_week_top_elements (
    game_week_id INTEGER NOT NULL REFERENCES game_weeks (id),
    player_id INTEGER NOT NULL REFERENCES players (id),
    points SMALLINT NOT NULL,
    PRIMARY KEY (game_week_id)
);
