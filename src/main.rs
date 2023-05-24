//Note i have copied and pasted this all and i am meerly going to study this and then discard this once i understand it and write something somewhat original. I am also using this project to learn Neovim on though i still will use vscode if i feel like it. also forgot that this is my first project that i am utilizing gitkraken instead of github desktop or vscode git integration.

// Import all that is needed.
use std::env;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token"); // token is set to whatever the contents of the Systems enviromnet variable titled DISCORD_TOKEN is.
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT; // I dont remeber what intents are but i need to set them. i will have to look into this.
    let mut client = Client::builder(token, intents) // Bot builder, actually contructs the bot instance
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why); // Hopefully you wont have to see this too often
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    println!("Received ping, Sending Pong!"); // output that the command has executed to console so i know what the fuck its doing.
    Ok(())
}
