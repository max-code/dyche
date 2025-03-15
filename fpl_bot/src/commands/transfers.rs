use std::time::Instant;

use crate::images::colours::GREY_COLOUR;
use crate::images::colours::PURPLE_COLOUR;
use crate::images::TransfersRenderer;
use crate::images::{PlayerGameInfo, PlayerInfo};
use fpl_common::types::{GameWeekId, LeagueId};
use fpl_db::queries::{
    game_week::get_current_game_week,
    mini_league::{get_league_name, get_team_ids_from_league_id},
    team::{get_team_ids_from_discord_ids, get_team_name_from_discord_id},
};
use tracing::debug;

use crate::{
    autocompletes::{autocomplete_league_or_user, autocomplete_league_or_user_value},
    commands::get_image_file_path,
    images::{Transfers, TransfersKey},
    log_call, log_timer, start_timer,
    utils::embed::{Embed, EmbedPage},
    Context, Error,
};

const COMMAND: &str = "/transfers";

#[poise::command(slash_command)]
pub async fn transfers(
    ctx: Context<'_>,
    #[description = "Transfers for a single user or entire league."]
    #[autocomplete = "autocomplete_league_or_user"]
    league_or_user: String,
    #[description = "User/League"]
    #[autocomplete = "autocomplete_league_or_user_value"]
    league_or_user_value: String,
    #[description = "Game Week"] game_week: Option<GameWeekId>,
) -> Result<(), Error> {
    log_call!(
        COMMAND,
        ctx,
        "league_or_user",
        league_or_user,
        "league_or_user_value",
        league_or_user_value,
        "game_week",
        game_week
    );
    let timer: Instant = start_timer!();

    let embed = Embed::from_ctx(ctx)?
        .processing()
        .title("Processing transfers request")
        .send()
        .await?;

    let game_week_id: i16 = match game_week {
        Some(gw) => i16::from(gw),
        None => i16::from(get_current_game_week(&ctx.data().pool).await?.id),
    };

    let value: i64 = league_or_user_value.parse::<i64>()?;

    let team_ids = match league_or_user.as_str() {
        "User" => {
            let ids = vec![value];
            let team_ids = get_team_ids_from_discord_ids(&ctx.data().pool, &ids).await?;
            log_timer!(timer, COMMAND, ctx, "got team_ids from discord_ids");
            team_ids
        }
        "League" => {
            let team_ids =
                get_team_ids_from_league_id(&ctx.data().pool, LeagueId::new(value as i32)).await?;
            log_timer!(timer, COMMAND, ctx, "got team_ids for ml");
            team_ids
        }
        _ => {
            return Err("Unknown league_or_user_type".into());
        }
    };

    let transfers = get_transfers(ctx, &team_ids, game_week_id).await?;

    let user_or_league_name = match league_or_user.as_str() {
        "User" => {
            let caller_name = get_team_name_from_discord_id(&ctx.data().pool, value).await?;
            log_timer!(timer, COMMAND, ctx, "got team names from discord_ids");
            caller_name
        }
        "League" => {
            let league_name =
                get_league_name(&ctx.data().pool, LeagueId::new(value as i32)).await?;
            log_timer!(timer, COMMAND, ctx, "got league name for ml");
            league_name
        }
        _ => {
            return Err("Unknown league_or_user_type".into());
        }
    };
    let title = format!(
        "Transfers for {} in GW{}",
        user_or_league_name, game_week_id
    );

    if transfers.user_to_transfers.is_empty() {
        embed
            .success()
            .title(title)
            .add_page(EmbedPage::new().add_row("No transfers."))
            .send()
            .await?;
        return Ok(());
    }

    let file_name = get_image_file_path(COMMAND, &ctx);
    let renderer = TransfersRenderer::default();
    renderer.render(transfers, &file_name).await?;
    log_timer!(timer, COMMAND, ctx, "rendered transfers image");

    embed
        .success()
        .title(title)
        .add_page(EmbedPage::new().with_image(file_name))
        .send()
        .await?;

    Ok(())
}

async fn get_transfers(
    ctx: Context<'_>,
    team_ids: &[i32],
    game_week_id: i16,
) -> Result<Transfers, Error> {
    let records = sqlx::query!(
        r#"
        SELECT 
            teams.player_first_name as "user_first_name",
            teams.player_last_name as "user_last_name",
            teams.name,
            player_in.web_name as "player_in_name!",
            player_in.code as "player_in_code!",
            po_in.opponents as "player_in_opponents",
            gwp_in.total_points as "player_in_points",
            gwp_in.minutes as "player_in_minutes",
            player_out.web_name as "player_out_name!",
            player_out.code as "player_out_code!",
            po_out.opponents as "player_out_opponents",
            gwp_out.total_points as "player_out_points",
            gwp_out.minutes as "player_out_minutes"
        FROM 
            transfers t
            join teams on teams.id = t.team_id 
            LEFT JOIN players player_in ON t.player_in_id = player_in.id
            LEFT JOIN players player_out ON t.player_out_id = player_out.id
            left join player_opponents po_in on po_in.player_id = player_in.id and po_in.game_week_id = $1
            left join player_opponents po_out on po_out.player_id = player_out.id and po_out.game_week_id = $1
            LEFT JOIN game_week_players gwp_in  ON player_in.id = gwp_in.player_id and gwp_in.game_week_id  = $1
   	        LEFT JOIN game_week_players gwp_out  ON player_out.id = gwp_out.player_id and gwp_out.game_week_id  = $1
            WHERE t.game_week_id = $1 and t.team_id = ANY($2);
        "#,
        game_week_id,
        team_ids
    ).fetch_all(&*ctx.data().pool).await?;

    let mut transfers = Transfers::new();

    for row in records {
        let key = TransfersKey {
            team_name: row.name,
            user_first_name: row.user_first_name,
            user_last_name: row.user_last_name,
        };

        let player_in_text = match row.player_in_minutes {
            0 => row.player_in_opponents.unwrap_or("N/A".to_string()),
            _ => row.player_in_points.to_string(),
        };

        // Grey them out if they arent playing. Will be active_bg_colour as we are using free text
        let player_in_bg_colour = match row.player_in_minutes {
            0 => GREY_COLOUR,
            _ => PURPLE_COLOUR,
        };

        let transfer_in = PlayerInfo::new(
            row.player_in_name,
            row.player_in_code as u32,
            vec![PlayerGameInfo::FreeText(player_in_text)],
            false,
            false,
        )
        .status_active_bg_color(player_in_bg_colour);

        let player_out_text = match row.player_out_minutes {
            0 => row.player_out_opponents.unwrap_or("N/A".to_string()),
            _ => row.player_out_points.to_string(),
        };

        // Grey them out if they arent playing. Will be active_bg_colour as we are using free text
        let player_out_bg_colour = match row.player_out_minutes {
            0 => GREY_COLOUR,
            _ => PURPLE_COLOUR,
        };

        let transfer_out = PlayerInfo::new(
            row.player_out_name,
            row.player_out_code as u32,
            vec![PlayerGameInfo::FreeText(player_out_text)],
            false,
            false,
        )
        .status_active_bg_color(player_out_bg_colour);

        transfers = transfers.add_transfer(key, transfer_out, transfer_in);
    }

    Ok(transfers)
}
