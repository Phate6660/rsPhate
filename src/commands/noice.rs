use log::{error, info};
use serenity::{
    framework::standard::{macros::command, CommandResult},
    http::AttachmentType,
    model::channel::Message,
    prelude::*,
};
use std::{fs, path::Path};
use std::process::Command;

#[command]
#[description = "Bot will post a random picture from Phate's \"just yes\" folder. Images will be spoiler'd, have fun!"]
fn noice(ctx: &mut Context, msg: &Message) -> CommandResult {
    let file = Command::new("scripts/noice")
        .arg("file")
        .output()
        .expect("Could not obtain random image");
    let path = format!("{}", String::from_utf8_lossy(&file.stdout).trim());
    let name = Command::new("scripts/noice")
        .arg("name")
        .arg(path)
        .output()
        .expect("Could not get base name.");
    let new_file = format!("images/just_yes/SPOILER_{}", String::from_utf8_lossy(&name.stdout).trim());
    fs::copy(String::from_utf8_lossy(&file.stdout).trim(), new_file).expect("could not copy file");
    let new_file2 = format!("images/just_yes/SPOILER_{}", String::from_utf8_lossy(&name.stdout).trim());
    let path2 = format!("{}", new_file2);
    
    let msg = msg.channel_id.send_message(&ctx.http, |m| {
        m.add_file(AttachmentType::Path(Path::new(&path2)));
        m
    });

    if let Err(why) = msg {
        error!("Error sending message: {:?}", why);
    }

    Ok(())
}
