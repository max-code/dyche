use chrono::{DateTime, Utc};
use fpl_api::responses::team::TeamResponse;
use fpl_common::types::{ClubId, GameWeekId, TeamId};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Team {
    pub id: TeamId,
    pub joined_time: DateTime<Utc>,
    pub started_event: GameWeekId,
    pub favourite_team: Option<ClubId>,
    pub player_first_name: String,
    pub player_last_name: String,
    pub player_region_id: i16,
    pub player_region_name: String,
    pub player_region_iso_code_short: String,
    pub player_region_iso_code_long: String,
    pub summary_overall_points: i16,
    pub summary_overall_rank: i32,
    pub summary_event_points: i16,
    pub summary_event_rank: Option<i32>,
    pub current_event: i16,
    pub name: String,
    pub name_change_blocked: bool,
    pub last_deadline_bank: i16,
    pub last_deadline_value: i16,
    pub last_deadline_total_transfers: i16,
}

impl From<TeamResponse> for Team {
    fn from(team: TeamResponse) -> Self {
        Self {
            id: team.id,
            joined_time: team.joined_time,
            started_event: team.started_event,
            favourite_team: team.favourite_team,
            player_first_name: team.player_first_name,
            player_last_name: team.player_last_name,
            player_region_id: team.player_region_id,
            player_region_name: team.player_region_name,
            player_region_iso_code_short: team.player_region_iso_code_short,
            player_region_iso_code_long: team.player_region_iso_code_long,
            summary_overall_points: team.summary_overall_points,
            summary_overall_rank: team.summary_overall_rank,
            summary_event_points: team.summary_event_points,
            summary_event_rank: team.summary_event_rank,
            current_event: team.current_event,
            name: team.name,
            name_change_blocked: team.name_change_blocked,
            last_deadline_bank: team.last_deadline_bank,
            last_deadline_value: team.last_deadline_value,
            last_deadline_total_transfers: team.last_deadline_total_transfers,
        }
    }
}

impl From<&TeamResponse> for Team {
    fn from(team: &TeamResponse) -> Self {
        Self {
            id: team.id,
            joined_time: team.joined_time,
            started_event: team.started_event,
            favourite_team: team.favourite_team,
            player_first_name: team.player_first_name.to_owned(),
            player_last_name: team.player_last_name.to_owned(),
            player_region_id: team.player_region_id,
            player_region_name: team.player_region_name.to_owned(),
            player_region_iso_code_short: team.player_region_iso_code_short.to_owned(),
            player_region_iso_code_long: team.player_region_iso_code_long.to_owned(),
            summary_overall_points: team.summary_overall_points,
            summary_overall_rank: team.summary_overall_rank,
            summary_event_points: team.summary_event_points,
            summary_event_rank: team.summary_event_rank,
            current_event: team.current_event,
            name: team.name.to_owned(),
            name_change_blocked: team.name_change_blocked,
            last_deadline_bank: team.last_deadline_bank,
            last_deadline_value: team.last_deadline_value,
            last_deadline_total_transfers: team.last_deadline_total_transfers,
        }
    }
}
