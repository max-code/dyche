-- Add migration script here
ALTER TABLE game_week_players
ALTER COLUMN player_id TYPE smallint,
ALTER COLUMN game_week_id TYPE smallint;
