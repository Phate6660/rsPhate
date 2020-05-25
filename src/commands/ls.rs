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
                ("**General**", "*Generalized functions for the bot.*", false),
                ("`^about`", "Information about the author and the bot. (But mostly the author.)", false),
                ("`^msg`", "Direct message user with list of commands.", false),
                ("`^quit`", "Bot will reply with \"Shutting down now!\" and shut itself down directly after.", false),
                ("`^date`", "Bot will reply with the date in the format -- `06:30 AM | Thu 21, May of 2020`.", false),
                ("`^projects`", "Bot will reply with pretty embed containing links to other project created/co-created by the author.", false),
                ("**Functions**", "*Functions for the bot that do not belong in any specific category.*", false),
                ("`^fortune`", "Display a random fortune from `fortune-mod-mythical-linux`.", false),
                ("`^hmm`", "How much music does Phate have?", false),
                ("`^iv SEARCH`", "Bot will reply with the first invidio link it can find related to the search query.", false),
                ("`^rr`", "Bot will reply with a link to Rick Astley's \"Never Gonna Give You Up\" without a link preview.", false),
                ("`^wipltrn`", "What is Phate listening to right now?", false),
                ("`^ww {apple,steam,systemd}`", "Bot will reply with pretty embed explaining why the topic is bad.", false),
                ("**Numbers**", "*Functions that are related to number operations.*", false),
                ("`^math operation num num`", "Bot will do math for you and send a message with the result.", false),
                ("`^rng min max`", "Bot will reply with a random number between the supplied min and max.", false),
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
