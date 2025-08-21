use std::collections::HashMap;

use serenity::all::Member;

use crate::app::commands::helpers;
use crate::domain::rank::falcon_rank::FalconRank;
use crate::domain::user::User;
use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn check_members(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;

    let members = helpers::get_members(ctx).await?;
    let discord_members = helpers::get_guild_members(ctx).await?;

    // Get the list of members not in the server or not in the google sheet
    let mut members_status: HashMap<u64, MemberStatus> = HashMap::new();

    for member in &discord_members {
        let user_id = member.user.id.get();

        let sheet = members.iter().any(|m| m.discord_id == user_id);
        members_status.insert(
            user_id,
            MemberStatus {
                server: true,
                sheet,
            },
        );
    }

    for member in &members {
        members_status
            .get(&member.discord_id)
            .get_or_insert(&MemberStatus {
                server: false,
                sheet: true,
            });
    }

    let not_in_server: Vec<&Member> = members_status
        .iter()
        .filter(|(_, status)| !status.server)
        .filter_map(|(user_id, _)| discord_members.iter().find(|m| m.user.id.get() == *user_id))
        .collect();

    let not_in_sheet: Vec<&Member> = members_status
        .iter()
        .filter(|(_, status)| !status.sheet)
        .filter_map(|(user_id, _)| discord_members.iter().find(|m| m.user.id.get() == *user_id))
        .collect();

    // Build the command response
    let mut response = String::new();

    response.push_str(generate_report(members).as_str());

    response.push_str("**ALERTS:**\n");
    if not_in_server.is_empty() && not_in_sheet.is_empty() {
        response.push_str("All members are in the server and the sheet.");
    } else {
        if !not_in_server.is_empty() {
            response.push_str("**NOT IN THE SERVER**\n");

            response.push_str(format_table(not_in_server).as_str());
            response.push_str("\n\n");
        }
        if !not_in_sheet.is_empty() {
            response.push_str("**NOT IN THE SHEET**\n");

            response.push_str(format_table(not_in_sheet).as_str());
            response.push_str("\n\n");
        }
    }

    ctx.say(response).await?;

    Ok(())
}

struct MemberStatus {
    pub server: bool,
    pub sheet: bool,
}

fn generate_report(members: Vec<User>) -> String {
    let mut report = String::new();

    report.push_str("**Report**\n");

    let mut rank_counter: HashMap<String, u32> = HashMap::new();

    for member in &members {
        let rank = member.current_rank.clone();
        let rank_count = rank_counter.entry(rank).or_insert(0);
        *rank_count += 1;
    }

    report.push_str(format!("{}: {}\n", "Total", members.len()).as_str());

    let falcon_ranks = vec![
        FalconRank::GodshardFalcon,
        FalconRank::DreadloFalcon,
        FalconRank::StarfireFalcon,
        FalconRank::LustreFalcon,
        FalconRank::VoidFalcon,
        FalconRank::DementiaFalcon,
        FalconRank::PlatinumFalcon,
        FalconRank::GoldFalcon,
        FalconRank::IronFalcon,
        FalconRank::CopperFalcon,
        FalconRank::GuildMember,
        FalconRank::NewMember,
        FalconRank::UnderGp,
        FalconRank::Kick,
    ];

    for fr in &falcon_ranks {
        let rank = fr.as_rank();
        report.push_str(
            format!(
                "{}: {}\n",
                rank.name,
                rank_counter.get(rank.code).unwrap_or(&0)
            )
            .as_str(),
        );
    }

    let mut invalid_ranks = 0;
    for code in rank_counter.keys() {
        if !falcon_ranks.iter().any(|fr| fr.as_rank().code == *code) {
            invalid_ranks += rank_counter.get(code).unwrap();
        }
    }
    report.push_str(format!("Invalid ranks on the sheet: {invalid_ranks}\n\n").as_str());

    report
}

fn format_table(members: Vec<&Member>) -> String {
    let mut table = String::new();
    table.push_str("```");

    table.push_str(format!("{:<19}  {}\n", "discord_id", "username").as_str());
    table.push_str(format!("{:-<19}  {:-<8}\n", "", "").as_str());

    for member in &members {
        let user_id = member.user.id.get();

        let username = member
            .nick
            .as_ref()
            .or(member.user.global_name.as_ref())
            .unwrap_or(&member.user.name)
            .clone();

        table.push_str(format!("{user_id:<19}  {username}\n").as_str());
    }
    table.push_str("```");

    table
}
