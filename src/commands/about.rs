use serenity::{
    framework::standard::{macros::command, CommandError, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
#[description = "Bot will reply with pretty embed containing title and description of bot, as well as where to find the author."]
fn about(ctx: &mut Context, msg: &Message) -> CommandResult {
    // Obtain Bot's profile pic: cache -> current info -> bot user -> bot icon
    let cache_http = &ctx.http;
    let current_info = match cache_http.get_current_application_info() {
        Ok(c) => c,
        Err(err) => return Err(CommandError(err.to_string())),
    };
    let bot_user = match current_info.id.to_user(cache_http) {
        Ok(u) => u,
        Err(err) => return Err(CommandError(err.to_string())),
    };
    let bot_icon = match bot_user.avatar_url() {
        Some(u) => u,
        None => bot_user.default_avatar_url(),
    };

    let msg = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("`rsPhate`");
            e.description("A bot with random and probably nonsensical features.\nWhere is the author, Phate6660?");
            e.thumbnail(bot_icon);
            // false = not inline
            e.fields(vec![
                ("Scripting/Programming", "[Github](https://github.com/Phate6660)", false),
                ("Social", "[Reddit](https://reddit.com/u/Valley6660), [Lobsters](https://lobste.rs/u/Phate6660)", false),
                ("Personal Site", "https://Phate6660.github.io/Main.html", false),
                ("Discord", "Phate6660#6270", false),
                ("Source Code", "[Phate6660/rsPhate](https://github.com/Phate6660/rsPhate)", false),
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
