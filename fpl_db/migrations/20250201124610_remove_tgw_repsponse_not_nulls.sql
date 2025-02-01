-- Add migration script here
ALTER TABLE team_game_weeks
ALTER COLUMN rank
DROP NOT NULL;

ALTER TABLE team_game_weeks
ALTER COLUMN rank_sort
DROP NOT NULL;

ALTER TABLE team_game_weeks
ALTER COLUMN percentile_rank
DROP NOT NULL;
