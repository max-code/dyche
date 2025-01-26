use fpl_api::responses::team_game_week::{AutomaticSub, Pick, TeamGameWeekResponse};
use fpl_common::types::{GameWeekId, PlayerId, TeamId};

#[derive(Debug, sqlx::FromRow)]
pub struct TeamGameWeek {
    pub team_id: TeamId,
    pub game_week_id: GameWeekId,
    pub active_chip: Option<String>,
    pub points: i16,
    pub total_points: i16,
    pub rank: i32,
    pub rank_sort: i32,
    pub overall_rank: i32,
    pub percentile_rank: i16,
    pub bank: i16,
    pub value: i16,
    pub event_transfers: i16,
    pub event_transfers_cost: i16,
    pub points_on_bench: i16,
}

impl From<(TeamId, GameWeekId, &TeamGameWeekResponse)> for TeamGameWeek {
    fn from(
        (team_id, game_week_id, response): (TeamId, GameWeekId, &TeamGameWeekResponse),
    ) -> Self {
        Self {
            team_id,
            game_week_id,
            active_chip: response.active_chip.map(|c| c.to_string()),
            points: response.entry_history.points,
            total_points: response.entry_history.total_points,
            rank: response.entry_history.rank,
            rank_sort: response.entry_history.rank_sort,
            overall_rank: response.entry_history.overall_rank,
            percentile_rank: response.entry_history.percentile_rank,
            bank: response.entry_history.bank,
            value: response.entry_history.value,
            event_transfers: response.entry_history.event_transfers,
            event_transfers_cost: response.entry_history.event_transfers_cost,
            points_on_bench: response.entry_history.points_on_bench,
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct TeamGameWeekPick {
    pub team_id: TeamId,
    pub game_week_id: GameWeekId,
    pub player_id: PlayerId,
    pub position: i16,
    pub multiplier: i16,
    pub is_captain: bool,
    pub is_vice_captain: bool,
    pub element_type: String,
}

impl From<(TeamId, GameWeekId, &Pick)> for TeamGameWeekPick {
    fn from((team_id, game_week_id, pick): (TeamId, GameWeekId, &Pick)) -> Self {
        Self {
            team_id,
            game_week_id,
            player_id: pick.element,
            position: pick.position,
            multiplier: pick.multiplier,
            is_captain: pick.is_captain,
            is_vice_captain: pick.is_vice_captain,
            element_type: pick.element_type.to_string(),
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct TeamGameWeekAutomaticSub {
    pub team_id: TeamId,
    pub game_week_id: GameWeekId,
    pub player_in_id: PlayerId,
    pub player_out_id: PlayerId,
}

impl From<&AutomaticSub> for TeamGameWeekAutomaticSub {
    fn from(sub: &AutomaticSub) -> Self {
        Self {
            team_id: sub.entry,
            game_week_id: sub.event,
            player_in_id: sub.element_in,
            player_out_id: sub.element_out,
        }
    }
}
