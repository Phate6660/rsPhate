use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};
use std::process::Command;

#[command]
fn date(ctx: &mut Context, msg: &Message) -> CommandResult {
    // `date` equals the output of the command `date +"%I:%M %p | %a %d, %b of %Y"`.
    let date = Command::new("date")
        .arg("+`%I:%M %p | %a %d, %b of %Y`")
        .output()
        .expect("Couldn't obtain current date and time.");
    // Sending a message can fail, due to a network error, an
    // authentication error, or lack of permissions to post in the
    // channel, so log to stdout when some error happens, with a
    // description of it.
    if let Err(why) = msg
        .channel_id
        .say(&ctx.http, String::from_utf8_lossy(&date.stdout))
    {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}
