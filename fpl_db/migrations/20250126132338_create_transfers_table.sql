-- Add migration script here
CREATE TABLE IF NOT EXISTS transfers (
    player_in_id SMALLINT NOT NULL REFERENCES players (id),
    player_out_id SMALLINT NOT NULL REFERENCES players (id),
    team_id SMALLINT NOT NULL REFERENCES teams (id),
    game_week_id SMALLINT NOT NULL REFERENCES game_weeks (id),
    player_in_cost SMALLINT NOT NULL,
    player_out_cost SMALLINT NOT NULL,
    transfer_time TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (
        player_in_id,
        player_out_id,
        team_id,
        game_week_id,
        transfer_time
    )
)
