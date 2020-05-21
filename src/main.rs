use std::env;
use std::process::Command;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
    utils::MessageBuilder,
};

struct Handler;

impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    fn message(&self, ctx: Context, msg: Message) {
        // If the user sends the message "^date", the bot replies with the current date and time.
        if msg.content == "^date" {
            // `date` equals the output of the command `date +"%I:%M %p | %a %d, %b of %Y"`.
            let date = Command::new("date")
                .arg("+`%I:%M %p | %a %d, %b of %Y`")
                .output()
                .expect("Couldn't obtain current date and time.");
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, String::from_utf8_lossy(&date.stdout)) {
                println!("Error sending message: {:?}", why);
            }
        }
        // If the user sends the message "^ww", the bot replies with who messaged and where.
        if msg.content == "^ww" {
            // Obtain channel.
            let channel = match msg.channel_id.to_channel(&ctx) {
                Ok(channel) => channel,
                Err(why) => {
                    println!("Error getting channel: {:?}", why);
                    return;
                }
            };

            // The message builder allows for creating a message by
            // mentioning users dynamically, pushing "safe" versions of
            // content (such as bolding normalized content), displaying
            // emojis, and more.
            let response = MessageBuilder::new()
                .push("User ")
                .push_bold_safe(&msg.author.name)
                .push(" used the `^ww` command in the ")
                .mention(&channel)
                .push(" channel")
                .build();

            if let Err(why) = msg.channel_id.say(&ctx.http, &response) {
                println!("Error sending message: {:?}", why);
            }
        }
        // Shutdown the bot if a user sends the message `^quit`.
        if msg.content == "^quit" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Shutting down now!") {
                println!("Error sending message: {:?}", why);
            }
            std::process::exit(0);
        }
        if msg.content == "^rr" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "<https://invidio.us/watch?v=dQw4w9WgXcQ&local=1>") {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
