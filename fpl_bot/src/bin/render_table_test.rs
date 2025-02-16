use fpl_bot::images::table::{TableData, TableRenderer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut data = TableData::new("Fantasy Premier League Standings".to_string());

    // Add test data
    data.add_row(
        "Max Jordan".to_string(),
        "BellinghamsDisciples".to_string(),
        26,
        1607,
        false,
    );
    data.add_row(
        "Kyran".to_string(),
        "cross & insalah".to_string(),
        26,
        1509,
        false,
    );
    data.add_row(
        "Sarah Smith".to_string(),
        "Hakimi Matata".to_string(),
        24,
        1485,
        true,
    );
    data.add_row(
        "James Wilson".to_string(),
        "No Kane No Gain".to_string(),
        28,
        1562,
        false,
    );
    data.add_row(
        "Emma Thompson".to_string(),
        "Moves Like Agger".to_string(),
        22,
        1423,
        false,
    );
    data.add_row(
        "Alex Rodriguez".to_string(),
        "Lord of the Ings".to_string(),
        25,
        1498,
        false,
    );
    data.add_row(
        "Michael Chen".to_string(),
        "Game of Throws".to_string(),
        27,
        1533,
        false,
    );
    data.add_row(
        "Jessica Brown".to_string(),
        "Kroos Control".to_string(),
        23,
        1445,
        false,
    );
    data.add_row(
        "David Martinez".to_string(),
        "Pique Blinders".to_string(),
        29,
        1589,
        false,
    );
    data.add_row(
        "Lisa Williams".to_string(),
        "Bale Force".to_string(),
        25,
        1476,
        false,
    );

    let renderer = TableRenderer::default();
    renderer.render(data, "league_standings.png").await?;

    println!("Table has been rendered successfully!");

    Ok(())
}
