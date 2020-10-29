use crate::ShardManagerContainer;
use crate::OWNER_CHECK;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    http::AttachmentType,
    model::channel::Message,
    prelude::*,
};
use std::path::Path;

#[command]
#[checks(Owner)]
#[aliases(exit, foad)]
fn quit(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();
    let message = &msg.content;

    if let Some(manager) = data.get::<ShardManagerContainer>() {
        // Shush, it'll make me feel better when I can't get the bot to work how I want.
        if message.contains("^foad") {
            let _ = msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title("I'm terribly sorry for being a failure.");
                    e.description("Expunging myself to robot hell as we speak.");
                    e.image("attachment://external-content.duckduckgo.com.gif");
                    e
                });
                m.add_file(AttachmentType::Path(Path::new("/home/valley/downloads/external-content.duckduckgo.com.gif")));
                m
            });
        } else {
            let _ = msg.reply(&ctx, "Hai, Phate-senpai~");
        }
        manager.lock().shutdown_all();
    } else {
        let _ = msg.reply(&ctx, "There was a problem getting the shard manager");
        return Ok(());
    }

    Ok(())
}
