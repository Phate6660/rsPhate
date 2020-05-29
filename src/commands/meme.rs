use log::{error, info};
use rofl;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    http::AttachmentType,
    model::channel::Message,
    prelude::*,
};
use std::{fs, io::Write, path::Path};

#[command]
#[description = "Bot will generate a meme based on input."]
#[usage = "position text"]
#[example = "bottom ah yes, enslaved meme generator"]
fn meme(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let position = args.single::<String>()?;
    let meme = args.rest();
    let pos = if position == "top" {
        rofl::VAlign::Top
    } else  {
        rofl::VAlign::Bottom
    };
    let engine = rofl::Engine::new("/home/valley/downloads/git/rofld/data/templates", "/home/valley/downloads/git/rofld/data/fonts");
    let image_macro = rofl::ImageMacro {
        template: "zoidberg".into(),
        captions: vec![
            rofl::Caption::text_at(pos, meme),
        ],
        ..rofl::ImageMacro::default()
    };
    let output = engine.caption(image_macro)?;
    info!("meme has been generated");
    
    let mut file = fs::OpenOptions::new().write(true).open("/home/valley/downloads/git/rofld/zoidberg.png")?;
    file.write_all(&*output)?;
    info!("meme has been written to /tmp/meme.png");

    let msg = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("rofl meme generator");
            e.image("attachment://zoidberg.png");
            e
        });
        m.add_file(AttachmentType::Path(Path::new("/home/valley/downloads/git/rofld/zoidberg.png")));
        m
    });

    if let Err(why) = msg {
        error!("Error sending message: {:?}", why);
    }

    Ok(())
}
