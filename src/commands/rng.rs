use rand::Rng;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
fn rng(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let min = args.single::<u32>()?; // First argument
    let max = args.single::<u32>()?; // Second argument

    let random_number = rand::thread_rng().gen_range(min, max); // generate a random number between min and max, ensure variable is a string

    if let Err(why) = msg.channel_id.say(&ctx.http, &random_number.to_string()) {
        println!("Err sending random number {}: {:?}", random_number, why);
    }

    Ok(())
}
