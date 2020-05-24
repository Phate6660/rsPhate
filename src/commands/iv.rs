use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::*,
};
use std::process::Command;

#[command]
fn iv(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let args = args.rest();

    let link = Command::new("scripts/iv")
        .arg(args)
        .output()
        .expect("Could not generate link.");

    if let Err(why) = msg
        .channel_id
        .say(&ctx.http, String::from_utf8_lossy(&link.stdout))
    {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}
