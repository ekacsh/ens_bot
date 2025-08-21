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

    let response = match member {
        Some(u) => {
            let current_rank = FalconRank::from_code(u.current_rank.as_ref())
                .unwrap()
                .as_rank();

            let mut response = String::new();
            response.push_str(format!("Username: {}\n", u.username).as_str());
            response.push_str(format!("Last Week GP: {}\n", u.week_gp).as_str());
            response.push_str(format!("Rank: {}\n", current_rank.name).as_str());
            response
        }
        None => "Member not found".to_string(),
    };

    ctx.say(response).await?;

    Ok(())
}
