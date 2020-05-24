use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
fn ls(ctx: &mut Context, msg: &Message) -> CommandResult {
    let msg = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("`^ls`");
            e.description("List available commands.");
            e.fields(vec![
                ("`^about`", "Information about the author and the bot. (But mostly the author.)", false),
                ("`^date`", "Bot will reply with the date in the format -- `06:30 AM | Thu 21, May of 2020`.", false),
                ("`^iv SEARCH`", "Bot will reply with the first invidio link it can find related to the search query.", false),
                ("`^msg`", "Direct message user with list of commands.", false),
                ("`^projects`", "Bot will reply with pretty embed containing links to other project created/co-created by the author.", false),
                ("`^rr`", "Bot will reply with a link to Rick Astley's \"Never Gonna Give You Up\" without a link preview.", false),
                ("`^wipltrn`", "What is Phate listening to right now?", false),
                ("`^ww {apple,steam,systemd}`", "Bot will reply with pretty embed explaining why the topic is bad.", false),
                ("`^quit`", "Bot will reply with \"Shutting down now!\" and shut itself down directly after.", false),
            ]);
            e
        });
        m
    });

    if let Err(why) = msg {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}
