use log::{error, info};
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::*,
    utils::MessageBuilder,
};

#[command]
#[description = "Bot will parse the input and output the correct full link to the repo."]
#[usage = "site,user/repo"]
#[example = "github,rsfetch/rsfetch"]
#[example = "gitlab,ArcticTheRogue/asgl"]
#[example = "codeberg,Phate6660/rsPhate"]
#[num_args(2)]
fn git(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let site = args.single::<String>()?;
    let repo = args.single::<String>()?;
    let match_site = site.as_str();

    // Log what was supplied
    info!("site: {}, owner/repo: {}", site, repo);

    // Match for site to create message.
    let message: String = match match_site {
        "github" => MessageBuilder::new()
            .push("https://github.com/")
            .push(repo)
            .build(),
        "gitlab" => MessageBuilder::new()
            .push("https://gitlab.com/")
            .push(repo)
            .build(),
        "codeberg" => MessageBuilder::new()
            .push("https://codeberg.org/")
            .push(repo)
            .build(),
        _ => "Could not generate a full link, please try again.".to_string(),
    };

    if let Err(why) = msg.channel_id.say(&ctx.http, &message) {
        error!("Could not push full Git repo link because: {}", why);
    }

    Ok(())
}
