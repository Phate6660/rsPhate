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
    let args: String = "ytsearch:".to_string() + &args.to_string();

    // todo: somehow obtain video id through a crate or something, maybe somebody made a youtube-dl crate?
    let id = Command::new("youtube-dl")
        .arg("--get-id")
        .arg(args)
        .output()
        .expect("Could not generate link.");
    info!("video id: {}", String::from_utf8_lossy(&id.stdout));

    let link: String =
        "https://invidio.us/watch?v=".to_string() + &String::from_utf8_lossy(&id.stdout);
    info!("invidio link: {}", link);

    if let Err(why) = msg.channel_id.say(&ctx.http, link) {
        error!("Error sending message: {:?}", why);
    }

    Ok(())
}
