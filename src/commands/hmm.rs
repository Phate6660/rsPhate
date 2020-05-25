use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};
use std::process::Command;

#[command]
fn hmm(ctx: &mut Context, msg: &Message) -> CommandResult {
    let artists = Command::new("scripts/hmm")
        .arg("artists")
        .output()
        .expect("Could not obtain amount of artists.");
    let albums = Command::new("scripts/hmm")
        .arg("albums")
        .output()
        .expect("Could not obtain amount of albums.");
    let songs = Command::new("scripts/hmm")
        .arg("songs")
        .output()
        .expect("Could not obtain amount of songs.");
    let files = Command::new("scripts/hmm")
        .arg("files")
        .output()
        .expect("Could not obtain amount of files.");
    let size = Command::new("scripts/hmm")
        .arg("size")
        .output()
        .expect("Could not obtain size of music collection.");
    let amount = Command::new("scripts/hmm")
        .arg("amount")
        .output()
        .expect("Could not obtain amount of times played.");
    let status = Command::new("scripts/hmm")
        .arg("status")
        .output()
        .expect("Could not obtain currently playing status.");
    let msg = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("`^hmm`");
            e.description("How Much Music (Does Phate Have?)");
            e.fields(vec![
                ("Artists", String::from_utf8_lossy(&artists.stdout), true),
                ("Albums", String::from_utf8_lossy(&albums.stdout), true),
                ("Songs", String::from_utf8_lossy(&songs.stdout), true),
                ("Files", String::from_utf8_lossy(&files.stdout), true),
                ("Size of Collection", String::from_utf8_lossy(&size.stdout), true),
                ("Amount of Songs Played", String::from_utf8_lossy(&amount.stdout), true),
                ("Currently Playing", String::from_utf8_lossy(&status.stdout), false),
            ]);
            e
        });
        m
    });

    if let Err(why) = msg {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}
