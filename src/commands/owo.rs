use log::error;
use owoify::OwOifiable;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
#[description = "Bot will reply with OwO-ified version of input."]
#[usage = "input"]
#[example = "// kill me, please -- phate"]
fn owo(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let input = String::from(args.rest());
    let owo_text = input.owoify();

    if let Err(why) = msg.channel_id.say(&ctx.http, owo_text) {
        error!("Error sending message: {:?}", why);
    }

    Ok(())
}
