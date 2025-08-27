use poise::CreateReply;

use crate::{app::commands::helpers, domain::{rank::falcon_rank::FalconRank, user::User}, Context, Error};

#[poise::command(slash_command)]
pub async fn to_kick(
    ctx: Context<'_>,
) -> Result<(), Error> {
    ctx.defer().await?;

    let members = helpers::get_members(ctx).await?;
    
    let members_to_kick: Vec<&User> = members.iter()
        .filter(|m| m.current_rank == FalconRank::Kick.as_rank().code)
        .collect();

    let mut lines = Vec::new();
    for member in members_to_kick {
        let user_id = serenity::all::UserId::from(member.discord_id);
        let user = ctx.serenity_context().http.get_user(user_id).await;
        let notes = match member.comments.trim() {
            "" => "",
            _ => " - ⚠️ Notice",
        };
        match user {
            Ok(u) => lines.push(format!("{} (<@{}>){}", u.name, u.id, notes)),
            Err(_) => lines.push(format!("Unknown User (ID: {}){}", member.discord_id, notes)),
        }
    }

    let response = if lines.is_empty() {
        "No members to kick.".to_string()
    } else {
        format!("Members to kick:\n```\n@silent\n{}```", lines.join("\n"))
    };
    
    let reply = CreateReply::default()
        .content(response)
        .allowed_mentions(serenity::all::CreateAllowedMentions::default().empty_users());
    
    ctx.send(reply).await?;

    Ok(())
}