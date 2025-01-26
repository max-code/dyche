-- Add migration script here
ALTER TABLE player_fixtures ADD PRIMARY KEY (player_id, fixture_id);
