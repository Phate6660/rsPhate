use rand::Rng;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
#[description = "Bot will generate a random number between min and max and reply with the result."]
#[usage = "min max"]
#[example = "-999 999"]
#[num_args(2)]
fn rng(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let min = args.single::<i64>()?; // First argument
    let max = args.single::<i64>()?; // Second argument

    let random_number = rand::thread_rng().gen_range(min, max); // generate a random number between min and max

    if let Err(why) = msg.channel_id.say(&ctx.http, &random_number.to_string()) {
        println!("Err sending random number {}: {:?}", random_number, why);
    }

    Ok(())
}
