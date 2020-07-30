use log::error;
use mpd::{Client, Stats};
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};
use std::process::Command;

#[command]
#[description = "How much music does Phate have?"]
fn hmm(ctx: &mut Context, msg: &Message) -> CommandResult {
    let mut c = Client::connect("127.0.0.1:6600").unwrap();
    let stats: Stats = c.stats().unwrap();
    let artists = stats.artists.to_string();
    let albums = stats.albums.to_string();
    let songs = stats.songs.to_string();
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
    let msg = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("`^hmm`");
            e.description("How Much Music (Does Phate Have?)");
            e.fields(vec![
                ("Artists", artists, true),
                ("Albums", albums, true),
                ("Songs", songs, true),
                (
                    "Files",
                    String::from_utf8_lossy(&files.stdout).to_string(),
                    true,
                ),
                (
                    "Size of Collection",
                    String::from_utf8_lossy(&size.stdout).to_string(),
                    true,
                ),
                (
                    "Amount of Songs Played",
                    String::from_utf8_lossy(&amount.stdout).to_string(),
                    true,
                ),
            ]);
            e
        });
        m
    });

    if let Err(why) = msg {
        error!("Error sending message: {:?}", why);
    }
    Ok(())
}
