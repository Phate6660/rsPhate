use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
fn quit(ctx: &mut Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.channel_id.say(&ctx.http, "Shutting down now!") {
        println!("Error sending message: {:?}", why);
    }

    std::process::exit(1); // Exit code of 1 to let myself know the bot was shutdown via command.
}
