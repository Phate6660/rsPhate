use date_time::{date_tuple::DateTuple, time_tuple::TimeTuple};
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
#[description = "Display the date in format: `06:30 AM | Mon 25, May of 2020`."]
fn date(ctx: &mut Context, msg: &Message) -> CommandResult {
    let date = DateTuple::today().to_readable_string();
    let time = TimeTuple::now().to_hhmm_string();
    let date = time + &" | ".to_string() + &date;

    if let Err(why) = msg.channel_id.say(&ctx.http, date) {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}
