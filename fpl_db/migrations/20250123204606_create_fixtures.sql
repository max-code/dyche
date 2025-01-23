CREATE TABLE fixtures (
    id INTEGER PRIMARY KEY,
    code INTEGER NOT NULL,
    game_week_id INTEGER NOT NULL,
    home_team_id INTEGER NOT NULL,
    away_team_id INTEGER NOT NULL,
    home_team_score SMALLINT,
    away_team_score SMALLINT,
    kickoff_time TIMESTAMPTZ NOT NULL,
    finished BOOLEAN NOT NULL DEFAULT false,
    started BOOLEAN NOT NULL DEFAULT false,
    minutes SMALLINT NOT NULL DEFAULT 0,
    provisional_start_time BOOLEAN NOT NULL DEFAULT false,
    team_h_difficulty SMALLINT NOT NULL,
    team_a_difficulty SMALLINT NOT NULL,
    pulse_id INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
);
