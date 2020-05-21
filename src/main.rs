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
        // Shutdown the bot.
        if msg.content == "^quit" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Shutting down now!") {
                println!("Error sending message: {:?}", why);
            }
            std::process::exit(1); // Exit code of 1 to let myself know the bot was shutdown via command.
        }
        // Send a link to a rick roll without a link preview.
        if msg.content == "^rr" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "<https://invidio.us/watch?v=dQw4w9WgXcQ&local=1>") {
                println!("Error sending message: {:?}", why);
            }
        }
        // Create a temporary invite.
        if msg.content == "^invite" {
            let channel = match ctx.cache.read().guild_channel(msg.channel_id) {
                Some(channel) => channel,
                None => {
                    let _ = msg.channel_id.say(&ctx, "Error creating invite");
                    return;
                }
            };

            let channel = channel.read();

            let creation = channel.create_invite(&ctx, |i| i.max_age(3600));

            let invite = match creation {
                Ok(invite) => invite,
                Err(why) => {
                    println!("Err creating invite: {:?}", why);
                    if let Err(why) = msg.channel_id.say(&ctx, "Error creating invite") {
                        println!("Err sending err msg: {:?}", why);
                    }

                    return;
                }
            };

            let content = format!("Here's your invite: {}", invite.url());
            let _ = msg.channel_id.say(&ctx, &content);
        }
        // List available commands.
        if msg.content == "^ls" {
            let msg = msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title("`^ls`");
                    e.description("List available commands.");
                    // false = not inline
					e.fields(vec![
                        ("`^date`", "Display the date in the format -- `06:30 AM | Thu 21, May of 2020`.", false),
						("`^invite`", "Create a 24 hour invite and send link in message.", false),
                        ("`^rr`", "Bot will reply with a link to Rick Astley's \"Never Gonna Give You Up\" without a link preview.", false),
						("`^ww`", "Bot will reply with -- \"User [insert bolded username here] used the ^ww command in the [insert channel mention here] channel\".", false),
						("`^quit`", "Bot will reply with \"Shutting down now!\" and shut itself down directly after.", false),
                    ]);
                    e
                });
                m
            });

            if let Err(why) = msg {
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
