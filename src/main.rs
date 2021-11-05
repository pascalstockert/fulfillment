use serenity::client::Client;
use serenity::framework::standard::{
    StandardFramework
};
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use dotenv::dotenv;
use std::env;

mod commands;
use commands::{
    help,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == help::HELP_COMMAND {
            help::help(&ctx, &msg).await;
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is alive!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let framework = StandardFramework::new();

    // Login with a bot token from the environment
    let token = env::var("BOT_TOKEN").expect("parsing token");

    let mut client = Client::builder(token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
