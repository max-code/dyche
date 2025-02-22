use chrono::{DateTime, Utc};
use fpl_common::types::{ClubId, FixtureId, GameWeekId, PlayerId};
use serde::Deserialize;
use std::collections::HashMap;

pub type FixturesResponse = Vec<GameweekFixture>;

#[derive(Debug)]
pub struct GameweekFixture {
    pub common: FixtureCommon,
    pub started: Option<bool>,
    pub team_h_difficulty: i16,
    pub team_a_difficulty: i16,
    pub pulse_id: i32,
    pub bonuses: Vec<GameWeekBonus>,
}

#[derive(Debug, Deserialize)]
pub struct FixtureCommon {
    pub id: FixtureId,
    pub code: i32,
    pub team_h: ClubId,
    pub team_h_score: Option<i16>,
    pub team_a: ClubId,
    pub team_a_score: Option<i16>,
    pub event: Option<GameWeekId>,
    pub finished: bool,
    pub minutes: i16,
    pub provisional_start_time: bool,
    pub kickoff_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct GameWeekBonus {
    pub fixture_id: FixtureId,
    pub player_id: PlayerId,
    pub bonus: i16,
    pub bps: i16,
}

impl<'de> Deserialize<'de> for GameweekFixture {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Helper structs for deserialization
        #[derive(Deserialize)]
        struct RawGameweekFixture {
            #[serde(flatten)]
            common: FixtureCommon,
            started: Option<bool>,
            team_h_difficulty: i16,
            team_a_difficulty: i16,
            pulse_id: i32,
            stats: Vec<StatEntry>,
        }

        #[derive(Deserialize)]
        struct StatEntry {
            identifier: String,
            a: Vec<StatValue>,
            h: Vec<StatValue>,
        }

        #[derive(Deserialize)]
        struct StatValue {
            value: i16,
            element: PlayerId,
        }

        let raw = RawGameweekFixture::deserialize(deserializer)?;
        let mut bonuses = HashMap::new();

        for stat in raw.stats {
            if stat.identifier == "bonus" || stat.identifier == "bps" {
                for team in [stat.a, stat.h] {
                    for stat_value in team {
                        let entry = bonuses.entry(stat_value.element).or_insert(GameWeekBonus {
                            fixture_id: raw.common.id,
                            player_id: stat_value.element,
                            bonus: 0,
                            bps: 0,
                        });

                        if stat.identifier == "bonus" {
                            entry.bonus += stat_value.value;
                        } else {
                            entry.bps += stat_value.value;
                        }
                    }
                }
            }
        }

        let bonuses = bonuses.into_values().collect();

        Ok(GameweekFixture {
            common: raw.common,
            started: raw.started,
            team_h_difficulty: raw.team_h_difficulty,
            team_a_difficulty: raw.team_a_difficulty,
            pulse_id: raw.pulse_id,
            bonuses,
        })
    }
}
