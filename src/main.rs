use medley_discord::commands::help;
use medley_discord::Data;
use medley_discord::Error;
use poise::builtins;
use poise::structs::FrameworkOptions;
use poise::Framework;
use poise::{serenity_prelude as serenity, FrameworkError};
use serenity::GatewayIntents;
use std::env;

#[tokio::main]
async fn main() {
    /* init */
    env::set_var("RUST_LOG", "warn,medley_discord=info");
    env_logger::init();
    dotenvy::dotenv().expect("Failed to read `.env` file.");
    let token = env::var("TOKEN").expect("Failed to read environment variable `TOKEN`.");

    /* setup */
    let options = FrameworkOptions {
        commands: vec![help::help()],
        on_error: |error| Box::pin(on_error(error)),
        ..Default::default()
    };
    let framework = Framework::builder()
        .token(token)
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                log::info!("Logged in as {}", _ready.user.name);
                builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .options(options)
        .intents(GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT);

    /* run */
    framework.run().await.unwrap();
}

async fn on_error(error: FrameworkError<'_, Data, Error>) {
    match error {
        FrameworkError::Setup { error, .. } => {
            panic!("Failed to start bot: {:?}", error);
        }
        FrameworkError::Command { error, ctx } => {
            log::error!("Error in command `{}`: {:?}", ctx.command().name, error);
        }
        error => {
            if let Err(e) = builtins::on_error(error).await {
                log::error!("Error while handling error: {}", e);
            }
        }
    }
}
