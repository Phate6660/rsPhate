use log::{error, info};
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::*,
};
use std::process::Command;

#[command]
#[description = "Bot will reply with an invidio link of the search query."]
#[usage = "search"]
#[example = "mindless self indulgence unsociable"]
fn iv(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let mut args = args.rest();
    info!("search query: {}", args);
    let args = str::replace(args, " ", "+");
    info!("ytdl search query: {}", args);
    let args: String = args.to_string();

    let link = Command::new("scripts/iv")
        .arg(args)
        .output()
        .expect("Could not generate link.");

    info!("invidio link: {}", String::from_utf8_lossy(&link.stdout));

    if let Err(why) = msg
        .channel_id
        .say(&ctx.http, String::from_utf8_lossy(&link.stdout))
    {
        error!("Error sending message: {:?}", why);
    }

    Ok(())
}
