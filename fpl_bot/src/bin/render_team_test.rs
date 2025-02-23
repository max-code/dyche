use fpl_bot::images::team::{TeamData, TeamRenderer};
use fpl_bot::images::util::{PlayerGameInfo, PlayerInfo};
use fpl_bot::images::GameStatus;
use fpl_common::types::{Chip, GameWeekId};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let team = TeamData::builder()
        .team_name("BellinghamsDisciples")
        .gw_rank(123456)
        .overall_rank(234567)
        .points(112)
        .game_week(GameWeekId::new(26)?)
        // Keeper
        .goalkeeper(PlayerInfo::new(
            "Alexander-Arnold".to_string(),
            118748,
            vec![PlayerGameInfo::Status(GameStatus::Played(6))],
            false,
            false,
            false,
        ))
        // Defenders
        .add_defender(PlayerInfo::new(
            "Salah".to_string(),
            118748,
            vec![PlayerGameInfo::Fixture("NEW (H)".to_string())],
            false,
            false,
            false,
        ))
        .add_defender(PlayerInfo::new(
            "Salah".to_string(),
            118748,
            vec![PlayerGameInfo::Status(GameStatus::NotPlayed)],
            false,
            false,
            false,
        ))
        .add_defender(PlayerInfo::new(
            "Salah".to_string(),
            118748,
            vec![PlayerGameInfo::Fixture("ARS (A)".to_string())],
            false,
            false,
            false,
        ))
        // Midfielders
        .add_midfielder(PlayerInfo::new(
            "Salah".to_string(),
            118748,
            vec![PlayerGameInfo::Status(GameStatus::Played(12))],
            true,
            false,
            true,
        ))
        .add_midfielder(PlayerInfo::new(
            "Salah".to_string(),
            118748,
            vec![
                PlayerGameInfo::Status(GameStatus::NotPlayed),
                PlayerGameInfo::Fixture("NFO (H)".to_string()),
            ],
            false,
            true,
            false,
        ))
        .add_midfielder(PlayerInfo::new(
            "Salah".to_string(),
            118748,
            vec![PlayerGameInfo::Status(GameStatus::Played(6))],
            false,
            false,
            false,
        ))
        .add_midfielder(PlayerInfo::new(
            "Salah".to_string(),
            118748,
            vec![PlayerGameInfo::Fixture("TOT (A)".to_string())],
            false,
            false,
            false,
        ))
        .add_midfielder(PlayerInfo::new(
            "Salah".to_string(),
            118748,
            vec![PlayerGameInfo::Status(GameStatus::Played(6))],
            false,
            false,
            false,
        ))
        // Forwards
        .add_forward(PlayerInfo::new(
            "Salah".to_string(),
            118748,
            vec![PlayerGameInfo::Status(GameStatus::Played(6))],
            false,
            false,
            false,
        ))
        .add_forward(PlayerInfo::new(
            "Salah".to_string(),
            118748,
            vec![PlayerGameInfo::Fixture("CHE (H)".to_string())],
            false,
            false,
            false,
        ))
        // Bench
        .add_bench_player(PlayerInfo::new(
            "Salah".to_string(),
            118748,
            vec![PlayerGameInfo::Status(GameStatus::NotPlayed)],
            false,
            false,
            false,
        ))
        .add_bench_player(PlayerInfo::new(
            "Salah".to_string(),
            118748,
            vec![PlayerGameInfo::Fixture("EVE (A)".to_string())],
            false,
            false,
            false,
        ))
        .add_bench_player(PlayerInfo::new(
            "Salah".to_string(),
            118748,
            vec![
                PlayerGameInfo::Status(GameStatus::Played(6)),
                PlayerGameInfo::Status(GameStatus::NotPlayed),
            ],
            false,
            false,
            false,
        ))
        .add_bench_player(PlayerInfo::new(
            "Salah".to_string(),
            118748,
            vec![
                PlayerGameInfo::Status(GameStatus::Played(6)),
                PlayerGameInfo::Fixture("BHA (H)".to_string()),
            ],
            false,
            false,
            false,
        ))
        .add_manager(PlayerInfo::new(
            "Slot".to_string(),
            100052173,
            vec![PlayerGameInfo::Fixture("BHA (H)".to_string())],
            false,
            false,
            false,
        ))
        .add_chip(Chip::AssMan)
        .build()?;

    let renderer = TeamRenderer::default();
    renderer.render(team, "team_display.png").await?;
    println!("Team has been rendered successfully!");
    Ok(())
}
