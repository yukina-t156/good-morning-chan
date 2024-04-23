
use anyhow::Context as _;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_runtime::SecretStore;
use tracing::{error, info};
use rand::prelude::*;


mod weather;

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
            let mut rng = StdRng::from_entropy(); // デフォルトの乱数生成器を初期化します
            let mut oha_msg = String::from("おはよう！<:ohayou:1231070930944397342>");
            if rng.gen_range(0..30)>25 {
                for _ in 0..40 {
                    oha_msg += "<:ohayou:1231070930944397342>" // おはようのカスタム絵文字のID
                }   
            }
            if let Err(e) = msg.channel_id.say(&ctx.http, oha_msg).await {
                error!("Error sending message: {:?}", e);
            }
        }
        if msg.content == "/おはようのオーディションして" {
            let mut rng = StdRng::from_entropy(); // デフォルトの乱数生成器を初期化します
            let result = match rng.gen_range(0..100) {
                0..=44 => String::from("合格"),
                45..=69 => String::from("不合格"),
                70..=84 => String::from("失格"),
                85..=89 => String::from("出禁"),
                90..=99 => String::from("優勝"),
                _ => String::from("エラー"),
            };
            if let Err(e) = msg.channel_id.say(&ctx.http, result).await {
                error!("Error sending message: {:?}", e);
            }
        }
        if msg.content == "/今日の天気" {
            
            let id = String::from("130010");
            let url = format!("https://weather.tsukumijima.net/api/forecast/city/{}", id);
            let data = weather::get_weather(&url).await;

            let result = match data {
                Ok(data) => {
                    // 成功した場合の処理
                    data
                }
                Err(_) => {
                    // エラーの場合の処理
                    String::from("API call is Failed. ごめんね！")
                }
            };
            

            if let Err(e) = msg.channel_id.say(&ctx.http, result).await {
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
