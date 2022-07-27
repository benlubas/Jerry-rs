use std::env;

use rand::Rng;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        let prefix: String = msg.content.chars().take(2).collect();
        if prefix != ">>" {
            return;
        }
        let message: String = msg.content.chars().skip(2).collect();
        match message.as_str().trim() {
            "ping" => {
                if let Err(why) = msg.channel_id.say(&ctx.http, "pong!").await {
                    println!("Error sending message: {:?}", why);
                }
            }
            "remind" => {
                if let Err(why) = msg
                    .channel_id
                    .say(&ctx.http, "@goomfy buy a controller")
                    .await
                {
                    println!("Error sending message: {:?}", why);
                }
            }
            "rps" => {
                let x: f32 = rand::prelude::random();
                let res = if x <= 0.33 {
                    "You lost"
                } else if x <= 0.66 {
                    "You win"
                } else {
                    "We tied"
                };

                if let Err(why) = msg.channel_id.say(&ctx.http, res).await {
                    println!("Error sending message: {:?}", why);
                }
            }
            _ => {}
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
