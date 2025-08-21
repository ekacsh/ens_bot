use std::collections::BTreeMap;

use crate::app::commands::helpers;
use crate::domain::user::User;
use crate::{Context, Error};

/// Shows the top 5 of the week ranking
#[poise::command(slash_command)]
pub async fn week_ranking(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;

    let top_ranking_count = 5;

    let members = helpers::get_members(ctx).await?;

    // Group members by week_gp
    let mut ranking: BTreeMap<u64, Vec<User>> = BTreeMap::new();

    for member in members {
        let rank = ranking.entry(member.week_gp).or_default();
        rank.push(member);
    }

    // Build the command response
    let mut response = String::new();
    response.push_str(format!("**Top {top_ranking_count}**\n").as_str());

    let mut place = 1;
    for (value, members) in ranking.iter().rev().take(top_ranking_count) {
        let mut sorted_members: Vec<String> = members.iter().map(|u| u.username.clone()).collect();
        sorted_members.sort();

        response.push_str(
            format!(
                "{:<2}. {:>4} - {}\n",
                place,
                value,
                sorted_members.join(", ")
            )
            .as_str(),
        );

        place += 1;
    }

    ctx.say(response).await?;

    Ok(())
}
