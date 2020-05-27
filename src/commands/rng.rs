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
    let message: String = "Could not generate a random number, please try again.".to_string();
    if min == 9223372036854775807 {
        let message_specific: String = "The min value supplied is invalid.".to_string();
        error!("{}", message_specific);
        msg.channel_id.say(&ctx.http, message_specific);
        msg.channel_id.say(&ctx.http, message);
        return Ok(());
    } else if max == 9223372036854775807 {
        let message_specific: String = "The max value supplied is invalid.".to_string();
        error!("{}", message_specific);
        msg.channel_id.say(&ctx.http, message_specific);
        msg.channel_id.say(&ctx.http, message);
        return Ok(());
    } else if min == max {
        let message_specific: String =
            "The min value must not be equal to, or greater than, the max value.".to_string();
        error!("{}", message_specific);
        msg.channel_id.say(&ctx.http, message_specific);
        msg.channel_id.say(&ctx.http, message);
        return Ok(());
    } else if min > max {
        let message_specific: String =
            "The min value must be lower than the max value.".to_string();
        error!("{}", message_specific);
        msg.channel_id.say(&ctx.http, message_specific);
        msg.channel_id.say(&ctx.http, message);
        return Ok(());
    }

    let random_number = rand::thread_rng().gen_range(min, max);

    if let Err(why) = msg.channel_id.say(&ctx.http, &random_number.to_string()) {
        error!("Could not send randomly generated number because: {}", why);
        msg.channel_id.say(
            &ctx.http,
            "Could not generate a random number, please try again.",
        );
    }

    Ok(())
}
