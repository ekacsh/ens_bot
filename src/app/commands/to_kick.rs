use poise::CreateReply;

use crate::{app::commands::helpers, domain::{rank::falcon_rank::FalconRank}, Context, Error};

#[poise::command(slash_command)]
pub async fn to_kick(
    ctx: Context<'_>,
) -> Result<(), Error> {
    ctx.defer().await?;

    let members = helpers::get_members(ctx).await?;
    
    let members_to_kick: Vec<u64> = members.iter()
        .filter(|m| m.current_rank == FalconRank::Kick.as_rank().code)
        .map(|m| m.discord_id)
        .collect();

    let response = 
        members_to_kick.iter().map(|id| format!("<@{id}>")).collect::<Vec<String>>().join("\n");

    let response = if response.is_empty() {
        "No members to kick.".to_string()
    } else {
        format!("Members to kick:\n{response}")
    };
    
    let reply = CreateReply::default()
        .content(response)
        .allowed_mentions(serenity::all::CreateAllowedMentions::default().empty_users());
    
    ctx.send(reply).await?;

    Ok(())
}