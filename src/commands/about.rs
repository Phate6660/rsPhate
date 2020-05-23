use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
fn about(ctx: &mut Context, msg: &Message) -> CommandResult {
    let msg = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("`rsPhate`");
            e.description("A bot with random and probably nonsensical features, made by Phate6660.\nWhere to find the author:");
            // false = not inline
			e.fields(vec![
                ("Github", "https://github.com/Phate6660", false),
				("Reddit", "https://reddit.com/u/Valley6660", false),
                ("Lobsters", "https://lobste.rs/u/Phate6660", false),
				("Personal Site", "https://Phate6660.github.io/Main.html", false),
				("Discord", "@Phate6660#6270", false),
				("Source Code", "https://github.com/Phate6660/rsPhate", false),
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
