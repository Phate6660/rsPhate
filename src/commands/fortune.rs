use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
    utils::MessageBuilder,
};
use std::process::Command;

#[command]
#[description = "Bot will reply with random fortune from `fortune-mod-mythical-linux` (repo is pinned to my profile if anyone is interested). (Note: Potentially NSFW and offensive fortunes are *enabled*, use at your own risk.)"]
fn fortune(ctx: &mut Context, msg: &Message) -> CommandResult {
    let fortune = Command::new("fortune")
        .arg("mythical_linux")
        .arg("off/mythical_linux")
        .output()
        .expect("Could not obtain fortune.");

    let response = MessageBuilder::new()
        .push("```\n")
        .push(String::from_utf8_lossy(&fortune.stdout))
        .push("\n```")
        .build();

    if let Err(why) = msg.channel_id.say(&ctx.http, response) {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}
