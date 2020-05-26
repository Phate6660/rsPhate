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

    if site == "hub" {
        let message = MessageBuilder::new()
            .push("https://github.com/")
            .push(repo)
            .build();
        if let Err(why) = msg.channel_id.say(&ctx.http, &message) {
            error!("Could not push full Github link because: {}", why);
        }
    } else if site == "lab" {
        let message = MessageBuilder::new()
            .push("https://gitlab.com/")
            .push(repo)
            .build();
        if let Err(why) = msg.channel_id.say(&ctx.http, &message) {
            error!("Could not push full Github link because: {}", why);
        }
    } else {
        let message: String = "Unknown git site or user error has occured, please try again.".to_string();
        error!("{}", message);
        if let Err(why) = msg.channel_id.say(&ctx.http, message) {
            error!("Could not push error message because: {}", why);
        }
    }

    Ok(())
}
