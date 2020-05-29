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
#[usage = "top_text bottom_text template"]
#[example = "ah yes,enslaved meme generator,anditsgone"]
fn meme(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let meme = args.single::<String>()?;
    let meme2 = args.single::<String>().unwrap_or("empty".to_string());
    let template = args.single::<String>().unwrap_or("zoidberg".to_string());

    let pos = if meme2 == "empty" {
        vec![rofl::Caption::text_at(rofl::VAlign::Top, meme)]
    } else {
        vec![
            rofl::Caption::text_at(rofl::VAlign::Top, meme),
            rofl::Caption::text_at(rofl::VAlign::Bottom, meme2),
        ]
    };
    let engine = rofl::Engine::new(
        "/home/valley/downloads/git/rofld/data/templates",
        "/home/valley/downloads/git/rofld/data/fonts",
    );
    let image_macro = rofl::ImageMacro {
        template: template.into(),
        captions: pos,
        ..rofl::ImageMacro::default()
    };
    let output = engine.caption(image_macro)?;
    info!("meme has been generated");

    let mut file = fs::OpenOptions::new()
        .write(true)
        .open("/tmp/meme.png")?;
    file.write_all(&*output)?;
    info!("meme has been written to /tmp/meme.png");

    let msg = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("rofl meme generator");
            e.image("attachment://meme.png");
            e
        });
        m.add_file(AttachmentType::Path(Path::new(
            "/tmp/meme.png",
        )));
        m
    });

    if let Err(why) = msg {
        error!("Error sending message: {:?}", why);
    }

    Ok(())
}
