-- Add migration script here
ALTER TABLE players
ALTER COLUMN ep_next TYPE REAL USING ep_next::real,
ALTER COLUMN ep_this TYPE REAL USING ep_this::real,
ALTER COLUMN form TYPE REAL USING NULLIF(form, '')::real,
ALTER COLUMN form DROP NOT NULL,
ALTER COLUMN points_per_game TYPE REAL USING ep_next::real,
ALTER COLUMN value_form TYPE REAL USING ep_next::real,
ALTER COLUMN value_season TYPE REAL USING ep_next::real,
ALTER COLUMN influence TYPE REAL USING ep_next::real,
ALTER COLUMN creativity TYPE REAL USING ep_next::real,
ALTER COLUMN threat TYPE REAL USING ep_next::real,
ALTER COLUMN expected_goals TYPE REAL USING ep_next::real,
ALTER COLUMN expected_assists TYPE REAL USING ep_next::real,
ALTER COLUMN expected_goal_involvements TYPE REAL USING ep_next::real,
ALTER COLUMN expected_goals_conceded TYPE REAL USING ep_next::real
