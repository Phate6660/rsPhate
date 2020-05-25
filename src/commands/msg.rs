use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
    utils::MessageBuilder,
};

#[command]
fn msg(ctx: &mut Context, msg: &Message) -> CommandResult {
    // Bot will send embed as DM.
    let dm = msg.author.dm(&ctx, |m| {
        m.content("Hello! Here's your personal list of commands!");
        m.embed(|e| {
            e.title("`^msg`");
            e.description("List available commands, but in a DM.");
            e.fields(vec![
                ("**General**", "*Generalized functions for the bot.*", false),
                ("`^about`", "Information about the author and the bot. (But mostly the author.)", false),
                ("`^ls`", "List available commands in a pretty embed.", false),
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

    if let Err(why) = dm {
        println!("Error when direct messaging user: {:?}", why);
    }

    // The message builder allows for creating a message by
    // mentioning users dynamically, pushing "safe" versions of
    // content (such as bolding normalized content), displaying
    // emojis, and more.
    //
    // Bot will send "Your message has been sent in a DM, **user**."
    let response = MessageBuilder::new()
        .push("Your message has been sent in a DM, ")
        .push_bold_safe(&msg.author.name)
        .push(".")
        .build();

    if let Err(why) = msg.channel_id.say(&ctx.http, &response) {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}
