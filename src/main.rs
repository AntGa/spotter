use chrono::{DateTime, Utc};
use poise::serenity_prelude::{self as serenity, Guild};
use serenity::{ChannelId, GuildId, UserId};

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(
    slash_command,
    install_context = "Guild | User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("pong!").await?;
    Ok(())
}

#[derive(Debug)]
struct RunPlan {
    planned_distance_m: Option<i32>,
}

#[derive(Debug)]
enum Activity {
    Gym,
    Run(RunPlan),
}

#[derive(Debug)]
struct Session {
    id: i64,
    name: String,
    activity: Activity,
    location: Option<String>,
    host: UserId,
    starts_at: DateTime<Utc>,
    created_at: DateTime<Utc>,
    cancelled_at: Option<DateTime<Utc>>,
    channel_id: ChannelId,
    guild_id: GuildId,
}

#[derive(Debug)]
struct Member {
    user_id: UserId,
    joined_at: DateTime<Utc>,
}

#[derive(Debug)]
struct SessionDetails {
    session: Session,
    members: Vec<Member>,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
