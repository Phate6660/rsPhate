use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
fn projects(ctx: &mut Context, msg: &Message) -> CommandResult {
    let msg = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Other Projects Created/Co-Created by The Author");
            e.fields(vec![
                ("rsfetch", "https://github.com/rsfetch/rsfetch", false),
                ("pkg", "https://github.com/Phate6660/pkg", false),
                ("p6nc-overlay", "https://github.com/p6nc/overlay", false),
                ("bindings", "https://github.com/Phate6660/bindings", false),
                ("sxhkd-bindings", "https://github.com/Phate6660/sxhkd-bindings", false),
                ("rust-server", "https://github.com/Phate6660/rust-server", false),
				("WBMPFMPD", "https://github.com/Phate6660/WBMPFMPD", false),
				("valleyTERM", "https://github.com/Phate6660/term", false),
				("FFNIFLFYTU", "https://github.com/Phate6660/FFNIFDBDFYTU", false),
				("cfg", "https://github.com/Phate6660/cfg", false),
				("fet", "https://github.com/Phate6660/fet", false),
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
