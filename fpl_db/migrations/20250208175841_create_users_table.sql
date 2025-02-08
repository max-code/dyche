-- Add migration script here
CREATE TABLE users (
    discord_id INTEGER PRIMARY KEY,
    team_id INTEGER REFERENCES teams (id)
);
