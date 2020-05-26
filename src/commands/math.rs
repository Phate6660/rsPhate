use log::error;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
#[description = "Bot will do math for you (basic add/sub/div/mul) and reply with the result."]
#[usage = "operation num num"]
#[example = "multiply 2 1"]
#[example = "divide 2 1"]
#[example = "add 1 1"]
#[example = "subtract 3 1"]
#[num_args(3)]
fn math(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let operation = args.single::<String>()?; // First argument, ensure it's a string

    if operation == "multiply" {
        let first_number = args.single::<f64>()?; // Second argument
        let second_number = args.single::<f64>()?; // Third argument

        let output = first_number * second_number;

        if let Err(why) = msg.channel_id.say(&ctx.http, &output.to_string()) {
            println!(
                "Err sending product of {} and {}: {:?}",
                first_number, second_number, why
            );
        }
    } else if operation == "divide" {
        let first_number = args.single::<f64>()?; // Second argument
        let second_number = args.single::<f64>()?; // Third argument

        let output = first_number / second_number;

        if let Err(why) = msg.channel_id.say(&ctx.http, &output.to_string()) {
            println!(
                "Err sending quotient of {} and {}: {:?}",
                first_number, second_number, why
            );
        }
    } else if operation == "add" {
        let first_number = args.single::<f64>()?; // Second argument
        let second_number = args.single::<f64>()?; // Third argument

        let output = first_number + second_number;

        if let Err(why) = msg.channel_id.say(&ctx.http, &output.to_string()) {
            println!(
                "Err sending addition of {} and {}: {:?}",
                first_number, second_number, why
            );
        }
    } else if operation == "subtract" {
        let first_number = args.single::<f64>()?; // Second argument
        let second_number = args.single::<f64>()?; // Third argument

        let output = first_number - second_number;

        if let Err(why) = msg.channel_id.say(&ctx.http, &output.to_string()) {
            println!(
                "Err sending subtraction of of {} and {}: {:?}",
                first_number, second_number, why
            );
        }
    } else {
        let message: String = "Unknown math operation or user error has occured, please try again.".to_string();
        error!("{}", message);
        if let Err(why) = msg.channel_id.say(&ctx.http, message) {
            error!("Could not push error message because: {}", why);
        }
    }

    Ok(())
}
