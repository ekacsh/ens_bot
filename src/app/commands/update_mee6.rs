use crate::{Context, Error};

// update-mee6
#[poise::command(slash_command)]
pub async fn update_mee6(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;

    // let members = get_members(ctx).await?;
    // let discord_members = get_guild_members(ctx).await?;

    ctx.say("Not available yet").await?;
    Ok(())
}
