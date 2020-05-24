use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
fn math(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let operation = args.single::<String>()?; // First argument

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
