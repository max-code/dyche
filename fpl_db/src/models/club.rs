use fpl_api::responses::game_state::ClubOverview;
use fpl_common::types::ClubId;

#[derive(Debug, sqlx::FromRow)]
pub struct Club {
    pub id: ClubId,
    pub code: i16,
    pub draw: i16,
    pub form: Option<String>,
    pub loss: i16,
    pub name: String,
    pub played: i16,
    pub points: i16,
    pub position: i16,
    pub short_name: String,
    pub strength: i16,
    pub team_division: Option<String>,
    pub unavailable: bool,
    pub win: i16,
    pub strength_overall_home: i16,
    pub strength_overall_away: i16,
    pub strength_attack_home: i16,
    pub strength_attack_away: i16,
    pub strength_defence_home: i16,
    pub strength_defence_away: i16,
    pub pulse_id: i16,
}

impl From<&ClubOverview> for Club {
    fn from(club: &ClubOverview) -> Self {
        Self {
            id: club.id,
            code: club.code,
            draw: club.draw,
            form: club.form.clone(),
            loss: club.loss,
            name: club.name.clone(),
            played: club.played,
            points: club.points,
            position: club.position,
            short_name: club.short_name.clone(),
            strength: club.strength,
            team_division: club.team_division.clone(),
            unavailable: club.unavailable,
            win: club.win,
            strength_overall_home: club.strength_overall_home,
            strength_overall_away: club.strength_overall_away,
            strength_attack_home: club.strength_attack_home,
            strength_attack_away: club.strength_attack_away,
            strength_defence_home: club.strength_defence_home,
            strength_defence_away: club.strength_defence_away,
            pulse_id: club.pulse_id,
        }
    }
}
