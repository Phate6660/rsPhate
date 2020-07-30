use log::{error, info};
use serenity::{
    framework::standard::{macros::command, CommandResult},
    http::AttachmentType,
    model::channel::Message,
    prelude::*,
};
use std::path::Path;
use std::process::Command;

#[command]
#[description = "Bot will post a random picture from Phate's \"just yes\" folder. May or may not post NSFW material. Use at own discretion."]
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
    let path2 = format!("{}", String::from_utf8_lossy(&file.stdout).trim());
    let attachment = format!("attachment://{}", String::from_utf8_lossy(&name.stdout).trim());
    info!(
        "\nfile: {}\nname: {}\nattachment: {}",
        String::from_utf8_lossy(&file.stdout).trim(),
        String::from_utf8_lossy(&name.stdout).trim(),
        attachment
    );
    let msg = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.image(attachment);
            e
        });
        m.add_file(AttachmentType::Path(Path::new(&path2)));
        m
    });

    if let Err(why) = msg {
        error!("Error sending message: {:?}", why);
    }

    Ok(())
}
