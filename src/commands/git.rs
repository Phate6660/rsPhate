use log::error;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::*,
    utils::MessageBuilder,
};

#[command]
#[description = "Bot will parse the input and output the correct full link to the repo."]
#[usage = "site user/repo"]
#[example = "hub Phate6660/rsPhate"]
#[example = "lab ArcticTheRogue/asgl"]
#[num_args(2)]
fn git(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let site = args.single::<String>()?;
    let repo = args.single::<String>()?;
    let match_site = site.as_str();

    // Match for site to create message.
    let message: String = match match_site {
        "hub" => {
            MessageBuilder::new()
                .push("https://github.com/")
                .push(repo)
                .build()
        },
        "lab" => {
            MessageBuilder::new()
                .push("https://gitlab.com/")
                .push(repo)
                .build()
        },
        _ => {
            "Could not generate a full link, please try again.".to_string()
        },
    };

    if let Err(why) = msg.channel_id.say(&ctx.http, &message) {
        error!("Could not push full Git* link because: {}", why);
    }

    Ok(())
}
