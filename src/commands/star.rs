use log::{error, info};
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
#[description = "Bot will star the user's message. Useful for starring your own messages with Hoshi bot."]
#[usage = "message"]
#[example = "message goes here"]
fn star(ctx: &mut Context, msg: &Message) -> CommandResult {
    info!(
        "starring the message: {}, sent by user: {}",
        msg.content, msg.author.name
    );
    if let Err(why) = msg.react(&ctx.http, '⭐') {
        error!("could not star the message");
    }

    Ok(())
}
