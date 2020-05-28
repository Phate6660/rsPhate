use date_time::{date_tuple::DateTuple, time_tuple::TimeTuple};
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
#[description = "Display the date in format: `14:47 | 28 May 2020`."]
fn date(ctx: &mut Context, msg: &Message) -> CommandResult {
    let date = DateTuple::today().to_readable_string(); // dd mmm yyyy
    let time = TimeTuple::now().to_hhmm_string(); // 00:00, 24-hour time
    let date = time + &" | ".to_string() + &date;

    if let Err(why) = msg.channel_id.say(&ctx.http, date) {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}
