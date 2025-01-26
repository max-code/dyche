use fpl_api::responses::game_week_players_stats::GameWeekPlayer;
use fpl_common::types::{GameWeekId, PlayerId};

#[derive(Debug, sqlx::FromRow)]
pub struct GameWeekPlayerDb {
    pub player_id: PlayerId,
    pub game_week_id: GameWeekId,
    pub minutes: i16,
    pub goals_scored: i16,
    pub assists: i16,
    pub clean_sheets: i16,
    pub goals_conceded: i16,
    pub own_goals: i16,
    pub penalties_saved: i16,
    pub penalties_missed: i16,
    pub yellow_cards: i16,
    pub red_cards: i16,
    pub saves: i16,
    pub bonus: i16,
    pub bps: i16,
    pub influence: f32,
    pub creativity: f32,
    pub threat: f32,
    pub ict_index: f32,
    pub starts: i16,
    pub expected_goals: f32,
    pub expected_assists: f32,
    pub expected_goal_involvements: f32,
    pub expected_goals_conceded: f32,
    pub total_points: i16,
    pub in_dreamteam: bool,
}

impl From<(GameWeekId, GameWeekPlayer)> for GameWeekPlayerDb {
    fn from((game_week_id, player): (GameWeekId, GameWeekPlayer)) -> Self {
        let stats = player.stats;
        Self {
            player_id: player.id,
            game_week_id,
            minutes: stats.minutes as i16,
            goals_scored: stats.goals_scored as i16,
            assists: stats.assists as i16,
            clean_sheets: stats.clean_sheets as i16,
            goals_conceded: stats.goals_conceded as i16,
            own_goals: stats.own_goals as i16,
            penalties_saved: stats.penalties_saved as i16,
            penalties_missed: stats.penalties_missed as i16,
            yellow_cards: stats.yellow_cards as i16,
            red_cards: stats.red_cards as i16,
            saves: stats.saves as i16,
            bonus: stats.bonus as i16,
            bps: stats.bps,
            influence: stats.influence,
            creativity: stats.creativity,
            threat: stats.threat,
            ict_index: stats.ict_index,
            starts: stats.starts as i16,
            expected_goals: stats.expected_goals,
            expected_assists: stats.expected_assists,
            expected_goal_involvements: stats.expected_goal_involvements,
            expected_goals_conceded: stats.expected_goals_conceded,
            total_points: stats.total_points as i16,
            in_dreamteam: stats.in_dreamteam,
        }
    }
}
