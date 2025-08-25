use std::collections::BTreeSet;

use chrono::{TimeZone, Utc};

use crate::{Context, Error, domain::rank::falcon_rank::FalconRank};

#[poise::command(slash_command)]
pub async fn member_info(
    ctx: Context<'_>,
    #[description = "The user to get the GP for"] user: Option<serenity::model::id::UserId>,
) -> Result<(), Error> {
    ctx.defer().await?;
    let members = ctx
        .data()
        .user_repository
        .as_ref()
        .get_users()
        .await
        .unwrap();

    let user = match user {
        Some(u) => &u.to_user(&ctx.http()).await?,
        None => ctx.author(),
    };

    let member = members.iter().find(|u| u.discord_id == user.id.get());

    let mut total_gp_sorted: Vec<_> = members.iter().collect();
    total_gp_sorted.sort_by(|a, b| b.total_gp.cmp(&a.total_gp));

    let response = match member {
        Some(u) => {
            
            let total_gp_position = {
                let ranking: BTreeSet<u64> = members.iter().map(|m| m.total_gp).collect();
                ranking.len() - ranking.iter().position(|x| *x == u.total_gp).unwrap()
            };

            let week_gp_position = {
                let ranking: BTreeSet<u64> = members.iter().map(|m| m.week_gp).collect();
                ranking.len() - ranking.iter().position(|x| *x == u.week_gp).unwrap()
            };

            let current_rank = FalconRank::from_code(u.current_rank.as_ref())
                .unwrap()
                .as_rank();

            let timestamp = u.joined_at.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp();

            let duration = Utc::now() - Utc.timestamp_opt(timestamp, 0).unwrap();
            let years = duration.num_days() / 365;

            let age_rank = match years {
                ..1 => "Newbie",
                1..2 => "Elder Falcon",
                2..3 => "Ancient Falcon",
                3..4 => "Ancestral Falcon",
                4..  => "Emberlight Falcon",

            };

            let mut response = String::new();
            response.push_str(format!("**Username**: {}\n", u.username).as_str());
            response.push_str(format!("**Last Week GP**: {} (#{week_gp_position})\n", u.week_gp).as_str());
            response.push_str(format!("**Total GP**: {} (#{total_gp_position})\n", u.total_gp).as_str());
            response.push_str(format!("**Rank**: {}\n", current_rank.name).as_str());
            response.push_str(format!("**Joined At**: <t:{timestamp}:d> ({age_rank})\n").as_str());
            response
        }
        None => "Member not found".to_string(),
    };

    ctx.say(response).await?;

    Ok(())
}
