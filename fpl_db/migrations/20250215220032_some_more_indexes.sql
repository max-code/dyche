-- Add migration script here
CREATE INDEX idx_clubs_code ON clubs(code);

CREATE INDEX idx_game_weeks_is_current ON game_weeks(is_current) WHERE is_current = true;
CREATE INDEX idx_game_weeks_is_next ON game_weeks(is_next) WHERE is_next = true;

CREATE INDEX idx_fixtures_game_week_id ON fixtures(game_week_id);
CREATE INDEX idx_fixtures_home_team_id ON fixtures(home_team_id);
CREATE INDEX idx_fixtures_away_team_id ON fixtures(away_team_id);
CREATE INDEX idx_fixtures_kickoff_time ON fixtures(kickoff_time);

CREATE INDEX idx_gw_players_total_points ON game_week_players(total_points);

CREATE INDEX idx_players_element_type ON players(element_type);
CREATE INDEX idx_players_total_points ON players(total_points);
CREATE INDEX idx_players_web_name ON players(web_name);
CREATE INDEX idx_players_first_name ON players(first_name);
CREATE INDEX idx_players_second_name ON players(second_name);
CREATE INDEX idx_players_first_second_name ON players(first_name, second_name);


CREATE INDEX idx_teams_summary_overall_rank ON teams(summary_overall_rank);
CREATE INDEX idx_teams_summary_event_rank ON teams(summary_event_rank);
CREATE INDEX idx_teams_name ON teams(name);
CREATE INDEX idx_teams_player_first_name ON teams(player_first_name);
CREATE INDEX idx_teams_player_last_name ON teams(player_last_name);

CREATE INDEX idx_transfers_team_id ON transfers(team_id);
CREATE INDEX idx_transfers_game_week_id ON transfers(game_week_id);
CREATE INDEX idx_transfers_player_in_out ON transfers(player_in_id, player_out_id);

CREATE INDEX idx_discord_users_team_id ON discord_users(team_id);

CREATE INDEX idx_mls_league_id ON mini_league_standings(league_id);
CREATE INDEX idx_mls_entry_name ON mini_league_standings(entry_name);
CREATE INDEX idx_mls_player_entry_name ON mini_league_standings(player_name,entry_name);
CREATE INDEX idx_mls_team_league_id ON mini_league_standings(team_id,league_id);
CREATE INDEX idx_mls_total ON mini_league_standings(total);

CREATE INDEX idx_player_fixtures_player_id ON player_fixtures(player_id);
CREATE INDEX idx_player_fixtures_fixture_id ON player_fixtures(fixture_id);

CREATE INDEX idx_tgwp_team_gw ON team_game_week_picks(team_id, game_week_id);
CREATE INDEX idx_tgwp_player_id ON team_game_week_picks(player_id);

CREATE INDEX idx_tgw_team_gw ON team_game_weeks(team_id, game_week_id);
CREATE INDEX idx_tgw_overall_rank ON team_game_weeks(overall_rank);