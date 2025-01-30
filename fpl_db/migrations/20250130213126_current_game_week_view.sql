-- Add migration script here
CREATE
OR REPLACE VIEW current_game_week AS
SELECT
    *
FROM
    game_weeks
WHERE
    deadline_time <= CURRENT_TIMESTAMP
ORDER BY
    deadline_time DESC
LIMIT
    1;

CREATE INDEX idx_game_weeks_deadline_time ON game_weeks (deadline_time DESC);
