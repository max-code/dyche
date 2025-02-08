-- Add migration script here
ALTER TABLE users
RENAME TO discord_users;

ALTER TABLE discord_users
ALTER COLUMN discord_id TYPE BIGINT;
