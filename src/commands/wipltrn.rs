use mpd::{Client, Song};
use serenity::{
    framework::standard::{macros::command, CommandResult},
    http::AttachmentType,
    model::channel::Message,
    prelude::*,
};
use std::path::Path;

#[command]
#[description = "Bot will reply with pretty embed containing current music info and cover art of what Phate is listening to."]
fn wipltrn(ctx: &mut Context, msg: &Message) -> CommandResult {
    let mut c = Client::connect("127.0.0.1:6600").unwrap();
    let song: Song = c.currentsong().unwrap().unwrap();
    let tit = song.title.as_ref().unwrap();
    let art = song.tags.get("Artist").unwrap();
    let alb = song.tags.get("Album").unwrap();
    let dat = song.tags.get("Date").unwrap();
    let mus_title = art.to_owned() + &" - ".to_string() + tit;
    let mus_album = alb.to_owned() + &" (".to_string() + &dat + &")".to_string();
    let msg = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title(&mus_title);
            e.description(&mus_album);
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
