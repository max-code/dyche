-- Add migration script here
CREATE TABLE IF NOT EXISTS clubs (
    id INTEGER PRIMARY KEY NOT NULL,
    code SMALLINT NOT NULL,
    draw SMALLINT NOT NULL,
    form TEXT,
    loss SMALLINT NOT NULL,
    name TEXT NOT NULL,
    played SMALLINT NOT NULL,
    points SMALLINT NOT NULL,
    position SMALLINT NOT NULL,
    short_name TEXT NOT NULL,
    strength SMALLINT NOT NULL,
    team_division TEXT,
    unavailable BOOLEAN NOT NULL DEFAULT false,
    win SMALLINT NOT NULL,
    strength_overall_home INTEGER NOT NULL,
    strength_overall_away INTEGER NOT NULL,
    strength_attack_home INTEGER NOT NULL,
    strength_attack_away INTEGER NOT NULL,
    strength_defence_home INTEGER NOT NULL,
    strength_defence_away INTEGER NOT NULL,
    pulse_id SMALLINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
);

-- Create an index on commonly queried fields
CREATE INDEX idx_clubs_name ON clubs (name);

CREATE INDEX idx_clubs_short_name ON clubs (short_name);
