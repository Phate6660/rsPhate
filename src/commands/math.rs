use log::{error, info};
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
    let pre_operation = args.single::<String>()?; // First argument, ensure it's a string
    let post_operation = pre_operation.as_str();

    match post_operation {
        "multiply" => {
            info!("Got operation: {}", pre_operation);
            let first_number = args.single::<i64>()?; // Second argument
            let second_number = args.single::<i64>()?; // Third argument

            let output = first_number * second_number;
            if let Err(why) = msg.channel_id.say(&ctx.http, &output.to_string()) {
                error!(
                    "Err sending product of {:?} and {:?}: {:?}",
                    first_number, second_number, why
                );
            }
        }
        "divide" => {
            info!("Got operation: {}", pre_operation);
            let first_number = args.single::<i64>()?; // Second argument
            let second_number = args.single::<i64>()?; // Third argument

            let output = first_number / second_number;
            if let Err(why) = msg.channel_id.say(&ctx.http, &output.to_string()) {
                error!(
                    "Err sending quotient of {:?} divided by {:?}: {:?}",
                    first_number, second_number, why
                );
            }
        }
        "add" => {
            info!("Got operation: {}", pre_operation);
            let first_number = args.single::<i64>()?; // Second argument
            let second_number = args.single::<i64>()?; // Third argument

            let output = first_number + second_number;
            if let Err(why) = msg.channel_id.say(&ctx.http, &output.to_string()) {
                error!(
                    "Err sending addition of {:?} by {:?}: {:?}",
                    first_number, second_number, why
                );
            }
        }
        "subtract" => {
            info!("Got operation: {}", pre_operation);
            let first_number = args.single::<i64>()?; // Second argument
            let second_number = args.single::<i64>()?; // Third argument

            let output = first_number - second_number;
            if let Err(why) = msg.channel_id.say(&ctx.http, &output.to_string()) {
                error!(
                    "Err sending subtraction of {:?} by {:?}: {:?}",
                    first_number, second_number, why
                );
            }
        }
        _ => {
            let message: String =
                "Unknown math operation or user error has occured, please try again.".to_string();
            error!("{}", message);
            if let Err(why) = msg.channel_id.say(&ctx.http, message) {
                error!("Could not push error message because: {}", why);
            }
        }
    }

    Ok(())
}
