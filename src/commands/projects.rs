use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
#[description = "Bot willk reply with pretty embed containing links to other projects by the author."]
fn projects(ctx: &mut Context, msg: &Message) -> CommandResult {
    let msg = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Other Projects Created/Co-Created by The Author");
            e.fields(vec![
                ("cfg", "https://github.com/Phate6660/cfg", false),
                ("gzdoom-discordrpc", "https://github.com/Phate6660/gzdoom-discordrpc", false),
                ("musinfo", "https://github.com/Phate6660/musinfo", false),
				("nixinfo", "https://github.com/Phate6660/rsnixinfo", false),
                ("p6nc-overlay", "https://github.com/p6nc/overlay", false),
                ("pkg", "https://github.com/Phate6660/pkg", false),
                ("rsfetch", "https://github.com/Phate6660/rsfetch", false),
                ("rsmpc", "https://github.com/Phate6660/rsmpc", false),
                ("rsmpv", "https://github.com/Phate6660/rsmpv", false),
                ("undeprecated-overlay", "https://github.com/Phate6660/undeprecated", false),
                ("WBMPFMPD", "https://github.com/Phate6660/WBMPFMPD", false),
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
