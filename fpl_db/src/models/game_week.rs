use chrono::{DateTime, Utc};
use fpl_api::responses::game_state::GameWeekOverview;
use fpl_common::types::{GameWeekId, PlayerId};

#[derive(Debug, sqlx::FromRow)]
pub struct GameWeek {
    pub id: GameWeekId,
    pub name: String,
    pub deadline_time: DateTime<Utc>,
    pub release_time: Option<DateTime<Utc>>,
    pub average_entry_score: i16,
    pub finished: bool,
    pub data_checked: bool,
    pub highest_scoring_entry: Option<i32>,
    pub deadline_time_epoch: i64,
    pub deadline_time_game_offset: i32,
    pub highest_score: Option<i16>,
    pub is_previous: bool,
    pub is_current: bool,
    pub is_next: bool,
    pub cup_leagues_created: bool,
    pub h2h_ko_matches_created: bool,
    pub can_enter: bool,
    pub can_manage: bool,
    pub released: bool,
    pub ranked_count: i32,
    pub transfers_made: i32,
    pub most_selected: Option<PlayerId>,
    pub most_transferred_in: Option<PlayerId>,
    pub top_element: Option<PlayerId>,
    pub most_captained: Option<PlayerId>,
    pub most_vice_captained: Option<PlayerId>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct GameWeekChipPlay {
    pub game_week_id: GameWeekId,
    pub chip_name: String,
    pub num_played: i32,
}

#[derive(Debug, sqlx::FromRow)]
pub struct GameWeekTopElement {
    pub game_week_id: GameWeekId,
    pub player_id: PlayerId,
    pub points: i16,
}

impl From<&GameWeekOverview> for GameWeek {
    fn from(gw: &GameWeekOverview) -> Self {
        Self {
            id: gw.id,
            name: gw.name.clone(),
            deadline_time: gw.deadline_time,
            release_time: gw.release_time,
            average_entry_score: gw.average_entry_score as i16,
            finished: gw.finished,
            data_checked: gw.data_checked,
            highest_scoring_entry: gw.highest_scoring_entry.map(|x| x as i32),
            deadline_time_epoch: gw.deadline_time_epoch,
            deadline_time_game_offset: gw.deadline_time_game_offset as i32,
            highest_score: gw.highest_score.map(|x| x as i16),
            is_previous: gw.is_previous,
            is_current: gw.is_current,
            is_next: gw.is_next,
            cup_leagues_created: gw.cup_leagues_created,
            h2h_ko_matches_created: gw.h2h_ko_matches_created,
            can_enter: gw.can_enter,
            can_manage: gw.can_manage,
            released: gw.released,
            ranked_count: gw.ranked_count as i32,
            transfers_made: gw.transfers_made as i32,
            most_selected: gw.most_selected,
            most_transferred_in: gw.most_transferred_in,
            top_element: gw.top_element,
            most_captained: gw.most_captained,
            most_vice_captained: gw.most_vice_captained,
        }
    }
}

impl GameWeekChipPlay {
    pub fn from_overview(game_week_overview: &GameWeekOverview) -> Vec<Self> {
        let chip_plays = &game_week_overview.chip_plays;

        chip_plays
            .iter()
            .map(|cp| Self {
                game_week_id: game_week_overview.id,
                chip_name: cp.chip_name.to_string(),
                num_played: cp.num_played as i32,
            })
            .collect()
    }
}

impl GameWeekTopElement {
    pub fn from_overview(game_week_overview: &GameWeekOverview) -> Option<Self> {
        game_week_overview
            .top_element_info
            .as_ref()
            .map(|info| Self {
                game_week_id: game_week_overview.id,
                player_id: info.id,
                points: info.points as i16,
            })
    }
}
