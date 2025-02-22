-- Add migration script here
CREATE TABLE bonus (
    fixture_id SMALLINT NOT NULL REFERENCES fixtures(id),
    player_id SMALLINT NOT NULL REFERENCES players(id),
    bps SMALLINT NOT NULL,
    bonus SMALLINT NOT NULL,
    PRIMARY KEY (fixture_id, player_id)
);
