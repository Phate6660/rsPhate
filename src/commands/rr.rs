use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
fn rr(ctx: &mut Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.channel_id.say(&ctx.http, "<https://invidio.us/watch?v=dQw4w9WgXcQ&local=1>") {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}
