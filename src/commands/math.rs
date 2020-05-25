use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
#[description = "Bot will do math for you (basic add/sub/div/mul) and reply with the result."]
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
    }

    if operation == "divide" {
        let first_number = args.single::<f64>()?; // Second argument
        let second_number = args.single::<f64>()?; // Third argument

        let output = first_number / second_number;

        if let Err(why) = msg.channel_id.say(&ctx.http, &output.to_string()) {
            println!(
                "Err sending quotient of {} and {}: {:?}",
                first_number, second_number, why
            );
        }
    }

    if operation == "add" {
        let first_number = args.single::<f64>()?; // Second argument
        let second_number = args.single::<f64>()?; // Third argument

        let output = first_number + second_number;

        if let Err(why) = msg.channel_id.say(&ctx.http, &output.to_string()) {
            println!(
                "Err sending addition of {} and {}: {:?}",
                first_number, second_number, why
            );
        }
    }

    if operation == "subtract" {
        let first_number = args.single::<f64>()?; // Second argument
        let second_number = args.single::<f64>()?; // Third argument

        let output = first_number - second_number;

        if let Err(why) = msg.channel_id.say(&ctx.http, &output.to_string()) {
            println!(
                "Err sending subtraction of of {} and {}: {:?}",
                first_number, second_number, why
            );
        }
    }

    Ok(())
}
