use poise::CreateReply;
use serenity::all::CreateAttachment;

use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn get_mee6_players(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    let members = ctx
        .data()
        .mee6_repository
        .as_ref()
        .get_players()
        .await
        .unwrap();

    let csv_content = {
        let mut csv = String::from("discord_id,level\n");
        for member in &members {
            csv.push_str(&format!("{},{}\n", member.discord_id, member.level));
        }
        csv
    };
    let filename = "mee6_players.csv";

    let attachment = CreateAttachment::bytes(csv_content.as_bytes(), filename);

    let reply = CreateReply::default()
        .content("Here is your CSV file:")
        .attachment(attachment);

    ctx.send(reply).await?;

    Ok(())
}
