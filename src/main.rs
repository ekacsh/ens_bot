use std::{env, sync::Arc};

use poise::serenity_prelude as serenity;

use ens_bot::{
    Data, Error,
    app::commands::{self, helpers},
    domain::{mee6_player::ApiMee6Repository, user::GSUserRepository},
};
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, fmt};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    init_tracing();

    let options = build_framework_options();
    let user_repository = build_user_repository();
    let mee6_repository = build_mee6_repository();

    let framework = build_framework(user_repository, mee6_repository, options).await;
    let mut client = build_client(framework).await;

    if let Err(e) = client.start().await {
        error!("Client error: {e:?}");
    }
}

fn init_tracing() {
    fmt().with_env_filter(EnvFilter::from_default_env()).init();
}

fn build_framework_options() -> poise::FrameworkOptions<Data, Error> {
    poise::FrameworkOptions {
        commands: vec![
            commands::ping(),
            commands::get_users(),
            commands::get_mee6_players(),
            commands::member_info(),
            commands::age_check(),
            commands::check_members(),
            commands::week_ranking(),
            commands::reset_ranks(),
            commands::to_kick(),
        ],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("!".into()),
            ..Default::default()
        },
        on_error: |error| Box::pin(on_error(error)),
        pre_command: |ctx| {
            Box::pin(async move {
                let username = ctx.author().name.clone();
                let user_id = ctx.author().id;
                info!("User {username}({user_id}) is executing the command {}!", ctx.command().qualified_name);
            })
        },
        command_check: Some(|ctx| {
            Box::pin(async move {
                if helpers::is_admin(ctx).await? {
                    return Ok(true);
                }

                let allow_all_commands = [commands::ping().name];

                let guild_member_commands =
                    [commands::member_info().name, commands::age_check().name];

                let is_guild_member = helpers::is_guild_member(ctx).await?;

                match &ctx.command().name {
                    name if allow_all_commands.contains(name) => Ok(true),
                    name if is_guild_member && guild_member_commands.contains(name) => Ok(true),
                    _ => Ok(false),
                }
            })
        }),
        post_command: |ctx| {
            Box::pin(async move {
                let username = ctx.author().name.clone();
                let user_id = ctx.author().id;
                info!("User {username}({user_id}) has executed the command {}!", ctx.command().qualified_name);
            })
        },
        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(async move {
                if let serenity::FullEvent::Ready { data_about_bot, .. } = event {
                    info!("Logged in as {}", data_about_bot.user.name);
                }
                Ok(())
            })
        },
        ..Default::default()
    }
}

fn build_user_repository() -> GSUserRepository {
    let api_url = env::var("GS_API_URL").expect("Expected the api url in the environment");
    GSUserRepository::new(api_url)
}

fn build_mee6_repository() -> ApiMee6Repository {
    let api_url = env::var("MEE6_API_URL").expect("Expected the mee6 api url in the environment");
    let auth_token =
        env::var("MEE6_TOKEN").expect("Expected the mee6 auth token in the environment");
    ApiMee6Repository::new(api_url, auth_token)
}

async fn build_framework(
    user_repository: GSUserRepository,
    mee6_repository: ApiMee6Repository,
    options: poise::FrameworkOptions<Data, Error>,
) -> poise::Framework<Data, Error> {
    poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
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
        .build()
}

async fn build_client(framework: poise::Framework<Data, Error>) -> serenity::Client {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::MESSAGE_CONTENT
        | serenity::GatewayIntents::GUILD_MEMBERS;

    serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .expect("Err creating client")
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {error:?}"),
        poise::FrameworkError::Command { error, ctx, .. } => {
            error!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                error!("Error while handling error: {e}")
            }
        }
    }
}
