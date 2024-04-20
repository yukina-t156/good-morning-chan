
use anyhow::Context as _;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_runtime::SecretStore;
use tracing::{error, info};
use rand::Rng;

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "/hello" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "world!").await {
                error!("Error sending message: {:?}", e);
            }
        }
        if msg.content == "/oha" {
            let mut rng = rand::thread_rng(); // デフォルトの乱数生成器を初期化します
            let mut oha_msg = "おはよう！";
            if(rng.gen()%3==0){
                oha_msg += "<:ohayou:1231070930944397342>" // おはようのカスタム絵文字のID
            }
            if let Err(e) = msg.channel_id.say(&ctx.http, oha_msg).await {
                error!("Error sending message: {:?}", e);
            }
        }
        if msg.content == "/good-morning" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "good morning!").await {
                error!("Error sending message: {:?}", e);
            }
        }
        if msg.content == "/おはよ" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "good morning!").await {
                error!("Error sending message: {:?}", e);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = secrets.get("DISCORD_TOKEN").context("'DISCORD_TOKEN' was not found")?;

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client.into())
}
