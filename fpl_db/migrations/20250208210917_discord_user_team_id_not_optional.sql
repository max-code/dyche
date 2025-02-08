-- Add migration script here
ALTER TABLE discord_users
ALTER COLUMN team_id
SET
    NOT NULL;
