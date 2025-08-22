use std::collections::HashMap;

use serenity::all::RoleId;
use tracing::info;

use crate::app::commands::helpers;
use crate::domain::rank::falcon_rank::FalconRank;
use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn reset_ranks(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;

    let members = helpers::get_members(ctx).await?;
    let discord_members = helpers::get_guild_members(ctx).await?;

    let mut on_the_server: HashMap<u64, bool> = HashMap::new();

    let mut error_message = String::new();

    for gm in &discord_members {
        let user_id = gm.user.id.get();
        let username = discord_members
            .iter()
            .find(|m| m.user.id.get() == user_id)
            .map(|m| m.user.global_name.as_ref().unwrap_or(&m.user.name))
            .unwrap();

        on_the_server.insert(user_id, true);

        let current_rank = {
            let code = match &members.iter().find(|u| u.discord_id == user_id) {
                Some(u) => u.current_rank.clone(),
                None => {
                    error_message.push_str(&format!("Member {user_id} not in the google sheet\n"));
                    continue;
                }
            };

            FalconRank::from_code(&code).unwrap()
        };

        let current_rank_id = current_rank.as_rank().rank_id;

        let rank_roles: Vec<&RoleId> = gm
            .roles
            .iter()
            .filter(|r| {
                let rank_id = r.get();
                FalconRank::RANK_ROLES
                    .iter()
                    .any(|rr| rr.as_rank().rank_id == rank_id)
            })
            .collect();

        let mut rank_updated = true;
        for rank_role in &rank_roles {
            if rank_role.get() == current_rank_id {
                rank_updated = false;
                continue;
            }

            info!(
                "[REMOVED_ROLE]: Removed role {} from {}({})",
                rank_role, username, gm.user.id
            );
            gm.remove_role(ctx, *rank_role).await.unwrap();
        }

        if current_rank == FalconRank::NewMember || current_rank == FalconRank::GuildMember {
            // New members don't get a rank and guild members don't have aditional role
            continue;
        }

        if rank_updated {
            let rank = current_rank.as_rank();

            gm.add_role(ctx, rank.rank_id).await.unwrap();
            info!(
                "[ADDED_ROLE]: Added role {}({}) to {}({})",
                rank.name, rank.rank_id, username, gm.user.id
            );
        }
    }

    for member in &members {
        if !on_the_server.contains_key(&member.discord_id) {
            error_message.push_str(
                format!(
                    "Member {}({}) not in the server\n",
                    member.username, member.discord_id
                )
                .as_str(),
            );
        }
    }

    ctx.say(error_message).await?;
    Ok(())
}
