use crate::{
    Context, Error,
    domain::{rank::falcon_rank::FalconRank, user::User},
};
use serenity::all::{Member, RoleId};

pub async fn get_members(ctx: Context<'_>) -> Result<Vec<User>, Error> {
    let repo = &ctx.data().user_repository;
    repo.invalidate_cache();

    repo.get_users().await
}

pub async fn get_guild_members(ctx: Context<'_>) -> Result<Vec<Member>, Error> {
    let guild_id = ctx.guild_id().unwrap();
    let http = ctx.serenity_context().http.clone();

    let mut discord_members = guild_id.members(http, None, None).await?;

    let guild_member_id = RoleId::new(FalconRank::GuildMember.as_rank().rank_id);

    discord_members.retain(|m| m.roles.contains(&guild_member_id));

    Ok(discord_members)
}

pub async fn is_admin(ctx: Context<'_>) -> Result<bool, Error> {
    has_rank(ctx, FalconRank::Admin).await
}

pub async fn is_guild_member(ctx: Context<'_>) -> Result<bool, Error> {
    has_rank(ctx, FalconRank::GuildMember).await
}

async fn has_rank(ctx: Context<'_>, rank: FalconRank) -> Result<bool, Error> {
    let http = ctx.serenity_context().http.clone();
    let guild_id = ctx.guild_id().unwrap();
    let role_id = RoleId::new(rank.as_rank().rank_id);

    Ok(ctx.author().has_role(http, guild_id, role_id).await?)
}
