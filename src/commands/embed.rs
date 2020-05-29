use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
#[description = "Bot will generate an embed based on input."]
#[usage = "title description <image_link>"]
#[example = "^embed,rsPhate can generate embeds!,<https://website.com/path/to/image.ext>"]
fn embed(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let title = args.single::<String>()?;
    let description = args.single::<String>()?;
    let mut image = args.single::<String>().unwrap_or("false".to_string());
    
    let link = if image == "false" {
        "https://i.imgur.com/pMBcpoq.png".to_string()
    } else {
        image.replace("<", "").replace(">", "")
    };
    
    let msg = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title(title);
            e.description(description);
            e.image(link)
        });
        m
    });

    if let Err(why) = msg {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}
