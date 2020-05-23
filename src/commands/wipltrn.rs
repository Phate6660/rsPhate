use std::path::Path;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    http::AttachmentType,
    model::channel::Message,
    prelude::*,
};
use std::process::Command;

#[command]
fn wipltrn(ctx: &mut Context, msg: &Message) -> CommandResult {
    // `mus_title` = artist - title
    let mus_title = Command::new("sh")
        .arg("-c")
        .arg("mpc -f \"%artist% - %title%\" | head -n1")
        .output()
        .expect("Could not obtain artist and title.");
    // `mus_album` = album (date)
    let mus_album = Command::new("sh")
        .arg("-c")
        .arg("mpc -f \"%album% (%date%)\" | head -n1")
        .output()
        .expect("Could not obtain album and date.");
    let msg = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title(String::from_utf8_lossy(&mus_title.stdout));
            e.description(String::from_utf8_lossy(&mus_album.stdout));
            e.image("attachment://cover.png");
            e
        });
        m.add_file(AttachmentType::Path(Path::new("/tmp/cover.png")));
        m
    });

    if let Err(why) = msg {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}
