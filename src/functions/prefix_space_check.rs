use log::{error, info};
use serenity::{model::channel::Message, prelude::*};

// Users commonly message things like "^^^ what he said", this will check if there's a "^ " in the message and will ignore it if it's found.
pub fn prefix_space_check(ctx: &mut Context, msg: &Message, unknown_command_name: &str) {
    if msg.content.contains("^ ") {
        info!("There was a space after the prefix, assuming the bot was not intended to be used.");
    } else {
        error!("Invalid command: '{}'", unknown_command_name);
        let msg = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.description("Invalid command, please use `^help` to check for valid commands.");
                e
            })
        });

        if let Err(why) = msg {
            error!("Error sending message: {:?}", why);
        }
    }
}
