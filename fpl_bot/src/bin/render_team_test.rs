use fpl_bot::images::team::{TeamData, TeamRenderer};
use fpl_bot::images::util::{PlayerGameInfo, PlayerInfo};
use fpl_common::types::Chip;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let team = TeamData::builder()
        .team_name("Hakimi Matata")
        .gw_rank(123456)
        .overall_rank(234567)
        // Keeper
        .goalkeeper(PlayerInfo::new(
            "Alexander-Arnold".to_string(),
            118748,
            vec![PlayerGameInfo::Points(6)],
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
            vec![PlayerGameInfo::Points(2)],
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
            vec![PlayerGameInfo::Points(12)],
            true,
            false,
            true,
        ))
        .add_midfielder(PlayerInfo::new(
            "Salah".to_string(),
            118748,
            vec![PlayerGameInfo::Fixture("MUN (H)".to_string())],
            false,
            true,
            false,
        ))
        .add_midfielder(PlayerInfo::new(
            "Salah".to_string(),
            118748,
            vec![PlayerGameInfo::Points(3)],
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
            vec![PlayerGameInfo::Points(8)],
            false,
            false,
            false,
        ))
        // Forwards
        .add_forward(PlayerInfo::new(
            "Salah".to_string(),
            118748,
            vec![PlayerGameInfo::Points(6)],
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
            vec![PlayerGameInfo::Points(0)],
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
            vec![PlayerGameInfo::Points(1)],
            false,
            false,
            false,
        ))
        .add_bench_player(PlayerInfo::new(
            "Salah".to_string(),
            118748,
            vec![PlayerGameInfo::Fixture("BHA (H)".to_string())],
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
