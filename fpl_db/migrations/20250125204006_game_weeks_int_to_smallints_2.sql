-- Add migration script here
ALTER TABLE game_week_top_elements
ALTER COLUMN game_week_id TYPE smallint,
ALTER COLUMN player_id TYPE smallint;

ALTER TABLE game_week_chip_plays
ALTER COLUMN game_week_id TYPE smallint;
