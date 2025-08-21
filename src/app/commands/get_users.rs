use poise::CreateReply;
use serenity::all::CreateAttachment;

use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn get_users(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;

    let repo = &ctx.data().user_repository;
    let members = repo.get_users().await?;

    let csv_content = {
        let mut csv = String::from("discord_id,username,week_gp\n");
        for member in &members {
            csv.push_str(&format!(
                "{},{},{}\n",
                member.discord_id, member.username, member.week_gp
            ));
        }
        csv
    };
    let filename = "users.csv";

    let attachment = CreateAttachment::bytes(csv_content.as_bytes(), filename);

    let reply = CreateReply::default()
        .content("Here is your CSV file:")
        .attachment(attachment);

    ctx.send(reply).await?;

    Ok(())
}
