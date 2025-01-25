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

impl TryFrom<ClubOverview> for Club {
    type Error = anyhow::Error;
    fn try_from(club: ClubOverview) -> Result<Self, Self::Error> {
        Ok(Self {
            id: club.id,
            code: club.code as i16,
            draw: club.draw as i16,
            form: club.form,
            loss: club.loss as i16,
            name: club.name,
            played: club.played as i16,
            points: club.points as i16,
            position: club.position as i16,
            short_name: club.short_name,
            strength: club.strength as i16,
            team_division: club.team_division,
            unavailable: club.unavailable,
            win: club.win as i16,
            strength_overall_home: club.strength_overall_home as i16,
            strength_overall_away: club.strength_overall_away as i16,
            strength_attack_home: club.strength_attack_home as i16,
            strength_attack_away: club.strength_attack_away as i16,
            strength_defence_home: club.strength_defence_home as i16,
            strength_defence_away: club.strength_defence_away as i16,
            pulse_id: club.pulse_id as i16,
        })
    }
}
