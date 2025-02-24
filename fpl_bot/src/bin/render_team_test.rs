use fpl_bot::images::team::{TeamData, TeamRenderer};
use fpl_bot::images::util::{PlayerGameInfo, PlayerInfo};
use fpl_bot::images::{GameStatus, TransferInfo};
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
            1,
        ))
        // Defenders
        .add_defender(PlayerInfo::new(
            "Salah".to_string(),
            118748,
            vec![PlayerGameInfo::Fixture("NEW (H)".to_string())],
            false,
            false,
            1,
        ))
        .add_defender(PlayerInfo::new(
            "Salah".to_string(),
            118748,
            vec![PlayerGameInfo::Status(GameStatus::NotPlayed)],
            false,
            false,
            1,
        ))
        .add_defender(PlayerInfo::new(
            "Salah".to_string(),
            118748,
            vec![PlayerGameInfo::Fixture("ARS (A)".to_string())],
            false,
            false,
            1,
        ))
        // Midfielders
        .add_midfielder(PlayerInfo::new(
            "Salah".to_string(),
            118748,
            vec![PlayerGameInfo::Status(GameStatus::Played(12))],
            true,
            false,
            2,
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
            1,
        ))
        .add_midfielder(PlayerInfo::new(
            "Salah".to_string(),
            118748,
            vec![PlayerGameInfo::Status(GameStatus::Played(6))],
            false,
            false,
            1,
        ))
        .add_midfielder(PlayerInfo::new(
            "Salah".to_string(),
            118748,
            vec![PlayerGameInfo::Fixture("TOT (A)".to_string())],
            false,
            false,
            1,
        ))
        .add_midfielder(PlayerInfo::new(
            "Salah".to_string(),
            118748,
            vec![PlayerGameInfo::Status(GameStatus::Played(6))],
            false,
            false,
            1,
        ))
        // Forwards
        .add_forward(PlayerInfo::new(
            "Salah".to_string(),
            118748,
            vec![PlayerGameInfo::Status(GameStatus::Played(6))],
            false,
            false,
            1,
        ))
        .add_forward(PlayerInfo::new(
            "Salah".to_string(),
            118748,
            vec![PlayerGameInfo::Fixture("CHE (H)".to_string())],
            false,
            false,
            1,
        ))
        // Bench
        .add_bench_player(PlayerInfo::new(
            "Salah".to_string(),
            118748,
            vec![PlayerGameInfo::Status(GameStatus::NotPlayed)],
            false,
            false,
            1,
        ))
        .add_bench_player(PlayerInfo::new(
            "Salah".to_string(),
            118748,
            vec![PlayerGameInfo::Fixture("EVE (A)".to_string())],
            false,
            false,
            1,
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
            1,
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
            1,
        ))
        .add_manager(PlayerInfo::new(
            "Slot".to_string(),
            100052173,
            vec![PlayerGameInfo::Fixture("BHA (H)".to_string())],
            false,
            false,
            1,
        ))
        .add_chip(Chip::AssMan)
        .add_transfer(TransferInfo::new(
            "Salah".to_string(),
            118748,
            10.9,
            "Saka".to_string(),
            223340,
            10.2,
        ))
        .add_transfer(TransferInfo::new(
            "Sels".to_string(),
            85633,
            5.4,
            "Raya".to_string(),
            154561,
            10.2,
        ))
        .add_transfer(TransferInfo::new(
            "Raya".to_string(),
            85633,
            5.4,
            "Sels".to_string(),
            154561,
            10.2,
        ))
        .build()?;

    let renderer = TeamRenderer::default();
    renderer.render(team, "team_display.png").await?;
    println!("Team has been rendered successfully!");
    Ok(())
}
