use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
    utils::MessageBuilder,
};
use std::process::Command;

#[command]
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
