use chrono::{DateTime, Utc};
use fpl_api::responses::mini_league::{MiniLeagueResponse, Standing};
use fpl_common::types::{GameWeekId, LeagueId, TeamId};

#[derive(Debug, sqlx::FromRow)]
pub struct MiniLeague {
    pub id: LeagueId,
    pub last_updated_data: DateTime<Utc>,
    pub name: String,
    pub created: DateTime<Utc>,
    pub closed: bool,
    pub max_entries: Option<i32>,
    pub league_type: String,
    pub scoring: String,
    pub admin_entry: TeamId,
    pub start_event: GameWeekId,
    pub code_privacy: String,
    pub has_cup: bool,
    pub cup_league: Option<i32>,
    pub rank: Option<i32>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct MiniLeagueStanding {
    pub id: i32,
    pub event_total: i16,
    pub player_name: String,
    pub rank: i32,
    pub last_rank: i32,
    pub rank_sort: i32,
    pub total: i16,
    pub team_id: TeamId,
    pub entry_name: String,
    pub has_player: bool,
    pub league_id: LeagueId,
}

impl From<&MiniLeagueResponse> for MiniLeague {
    fn from(mini_league: &MiniLeagueResponse) -> Self {
        Self {
            id: mini_league.league.id,
            last_updated_data: mini_league.last_updated_data,
            name: mini_league.league.name.clone(),
            created: mini_league.league.created,
            closed: mini_league.league.closed,
            max_entries: mini_league.league.max_entries,
            league_type: mini_league.league.league_type.clone(),
            scoring: mini_league.league.scoring.clone(),
            admin_entry: mini_league.league.admin_entry,
            start_event: mini_league.league.start_event,
            code_privacy: mini_league.league.code_privacy.clone(),
            has_cup: mini_league.league.has_cup,
            cup_league: mini_league.league.cup_league,
            rank: mini_league.league.rank,
        }
    }
}

impl From<(&LeagueId, &Standing)> for MiniLeagueStanding {
    fn from((league_id, standing): (&LeagueId, &Standing)) -> Self {
        Self {
            id: standing.id,
            event_total: standing.event_total,
            player_name: standing.player_name.clone(),
            rank: standing.rank,
            last_rank: standing.last_rank,
            rank_sort: standing.rank_sort,
            total: standing.total,
            team_id: standing.entry,
            entry_name: standing.entry_name.clone(),
            has_player: standing.has_played,
            league_id: league_id.clone(),
        }
    }
}
