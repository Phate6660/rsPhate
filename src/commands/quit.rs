use crate::OWNER_CHECK;
use crate::ShardManagerContainer;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
#[checks(Owner)]
#[description = "Bot will reply with \"Shutting down now!\" and shut itself down directly after."]
fn quit(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();

    if let Some(manager) = data.get::<ShardManagerContainer>() {
        let _ = msg.reply(&ctx, "Shutting down now!");
        manager.lock().shutdown_all();
    } else {
        let _ = msg.reply(&ctx, "There was a problem getting the shard manager");
        return Ok(());
    }

    Ok(())
}
