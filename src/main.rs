#![allow(deprecated)]
mod commands;
mod fraction_calc;

use std::collections::HashSet;
use std::env;
use std::io::Error;
use std::sync::Arc;

use tracing::{error, info};

use ask_gemini::Gemini;

use serenity::all::{Message, ReactionType};
use serenity::async_trait;
use serenity::framework::standard::{macros::group, Configuration};
use serenity::framework::StandardFramework;
use serenity::gateway::ShardManager;
use serenity::http::Http;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use crate::commands::fraction::FRACTION_COMMAND;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<ShardManager>;
}

const fn emoji(str: String) -> ReactionType {
    ReactionType::Unicode(str)
}

struct Handler(Gemini);

impl Handler {
    const PROMPT: &str = "次の行の文章に対する感想を絵文字一つで伝えてください";

    async fn gemini(&self, message: &str) -> std::io::Result<ReactionType> {
        let ask = &format!("{}\n{}", Self::PROMPT, message);

        match self.0.ask(ask).await {
            Ok(response) => {
                info!("Response: {:?}", response);
                let res = response[0].split(" ").collect::<Vec<&str>>();
                Ok(emoji(res[0].to_string()))
            }
            Err(e) => {
                error!("Error:: {}", e);
                Err(Error::new(std::io::ErrorKind::NotConnected, "gemini error"))
            }
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn message(&self, ctx: Context, message: Message) {
        let http = ctx.http.http();

        match message.channel(http).await {
            Ok(guild) => {
                if let Some(guild) = guild.guild() {
                    if guild.name != "test" {
                        return;
                    }
                }
            }
            Err(e) => {
                error!("{e}")
            }
        }
        if let Ok(emoji) = self.gemini(&message.content).await {
            if let Err(e) = message.react(http, emoji).await {
                error!("{}", e);
            }
        }
    }
}

#[group]
#[commands(fraction)]
struct General;

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let gemini = Gemini::new(None, None);

    let http = Http::new(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(owner) = &info.owner {
                owners.insert(owner.id);
            }

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    // Create the framework
    let framework = StandardFramework::new().group(&GENERAL_GROUP);
    framework.configure(Configuration::new().owners(owners).prefix("/"));

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler(gemini))
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
