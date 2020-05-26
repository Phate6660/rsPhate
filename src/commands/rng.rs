use log::error;
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
fn rng(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    fn min_and_max(mut a: Args) -> (i64, i64) {
        let min = a.single::<i64>().unwrap_or(9223372036854775807);
        let max = a.single::<i64>().unwrap_or(9223372036854775807);
        (min, max)
    }

    let (min, max) = min_and_max(args);
    if min == 9223372036854775807 {
        msg.channel_id.say(&ctx.http, "The min value supplied is invalid.");
        msg.channel_id.say(&ctx.http, "Could not generate a random number, please try again.");
        return Ok(())
    } else if max == 9223372036854775807 {
        msg.channel_id.say(&ctx.http, "The max value supplied is invalid.");
        msg.channel_id.say(&ctx.http, "Could not generate a random number, please try again.");
        return Ok(())
    }
    
    let random_number = rand::thread_rng().gen_range(min, max);

    if let Err(why) = msg.channel_id.say(&ctx.http, &random_number.to_string()) {
        error!("Could not send randomly generated number because: {}", why);
        msg.channel_id.say(&ctx.http, "Could not generate a random number, please try again.");
    }
    
    Ok(())
}
