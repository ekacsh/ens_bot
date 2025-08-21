use std::{env, sync::Arc};

use poise::serenity_prelude as serenity;

use ens_bot::{
    Data, Error,
    app::commands::{
        self,
        helpers::{is_admin, is_guild_member},
    },
    domain::{mee6_player::ApiMee6Repository, user::GSUserRepository},
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();

    let options = poise::FrameworkOptions {
        commands: vec![
            commands::ping(),
            commands::get_users(),
            commands::get_mee6_players(),
            commands::member_info(),
            commands::check_members(),
            commands::week_ranking(),
            commands::reset_ranks(),
        ],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("!".into()),
            ..Default::default()
        },
        on_error: |error| Box::pin(on_error(error)),
        pre_command: |ctx| {
            Box::pin(async move {
                println!("Executing command {}...", ctx.command().qualified_name);
            })
        },
        command_check: Some(|ctx| {
            Box::pin(async move {
                if is_admin(ctx).await? {
                    return Ok(true);
                }

                let is_guild_member = is_guild_member(ctx).await?;

                match ctx.command().name.as_str() {
                    "ping" => Ok(true),
                    "member_info" if is_guild_member => Ok(true),
                    _ => Ok(false),
                }
            })
        }),
        post_command: |ctx| {
            Box::pin(async move {
                println!("Executed command {}!", ctx.command().qualified_name);
            })
        },
        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(async move {
                match event {
                    serenity::FullEvent::Ready { data_about_bot, .. } => {
                        println!("Logged in as {}", data_about_bot.user.name);
                    }
                    _ => {}
                }
                Ok(())
            })
        },
        ..Default::default()
    };

    let user_repository = {
        let api_url = env::var("GS_API_URL").expect("Expected the api url in the environment");

        GSUserRepository::new(api_url)
    };

    let mee6_repository = {
        let api_url =
            env::var("MEE6_API_URL").expect("Expected the mee6 api url in the environment");
        let auth_token =
            env::var("MEE6_TOKEN").expect("Expected the mee6 auth token in the environment");

        ApiMee6Repository::new(api_url, auth_token)
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                // poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                // Remove all global commands
                // serenity::Command::set_global_commands(&ctx.http, vec![]).await?;

                let guild_id = env::var("GUILD_ID")
                    .expect("Expected the guild id in the environment")
                    .parse::<u64>()
                    .expect("Invalid guild id");

                let guild_id = serenity::GuildId::new(guild_id);

                poise::builtins::register_in_guild(ctx, &framework.options().commands, guild_id)
                    .await?;

                Ok(Data::new(
                    Arc::new(user_repository),
                    Arc::new(mee6_repository),
                ))
            })
        })
        .options(options)
        .build();

    let mut client = {
        let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

        let intents = serenity::GatewayIntents::non_privileged()
            | serenity::GatewayIntents::MESSAGE_CONTENT
            | serenity::GatewayIntents::GUILD_MEMBERS;

        serenity::ClientBuilder::new(token, intents)
            .framework(framework)
            .await
            .expect("Err creating client")
    };

    if let Err(e) = client.start().await {
        println!("Client error: {e:?}");
    }
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {error:?}"),
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {e}")
            }
        }
    }
}
