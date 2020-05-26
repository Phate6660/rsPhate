use crate::ShardManagerContainer;
use crate::OWNER_CHECK;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
#[checks(Owner)]
#[aliases(exit)]
fn quit(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();

    if let Some(manager) = data.get::<ShardManagerContainer>() {
        let _ = msg.reply(&ctx, "Hai, Phate-senpai~");
        manager.lock().shutdown_all();
    } else {
        let _ = msg.reply(&ctx, "There was a problem getting the shard manager");
        return Ok(());
    }

    Ok(())
}
