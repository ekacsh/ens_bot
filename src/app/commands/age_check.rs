use crate::{app::commands::helpers, Context, Error};

#[poise::command(slash_command)]
pub async fn age_check(
    ctx: Context<'_>,
    #[description = "The user to get the GP for"] user: Option<serenity::model::id::UserId>,
) -> Result<(), Error> {
    ctx.defer().await?;

    let members = helpers::get_members(ctx).await?;

    
    let user = match user {
        Some(u) => &u.to_user(&ctx.http()).await?,
        None => ctx.author(),
    };
    
    let member = members.iter().find(|u| u.discord_id == user.id.get());

    let member = match member {
        Some(m) => m,
        None => {
            ctx.say("Member not found.").await?;
            return Ok(());
        }
    };

    let timestamp = member.joined_at.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp();

    ctx.say(format!("{} has been a Guild Member since <t:{timestamp}:d> (<t:{timestamp}:R>).", user.name)).await?;

    Ok(())
}