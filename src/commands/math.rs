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
            
            if let output = first_number * second_number {
                if let Err(why) = msg.channel_id.say(&ctx.http, &output.to_string()) {
                    error!(
                        "Err sending product of {:?} and {:?}: {:?}",
                        first_number, second_number, why
                    );
                }
            } else {
                error!("Err multiplying {:?} and {:?}.", first_number, second_number);
                msg.channel_id.say(&ctx.http, "There was an error while multiplying.");
            }
        }
        "divide" => {
            info!("Got operation: {}", pre_operation);
            let first_number = args.single::<i64>()?; // Second argument
            let second_number = args.single::<i64>()?; // Third argument
            
            if let output = first_number / second_number {
                if let Err(why) = msg.channel_id.say(&ctx.http, &output.to_string()) {
                    error!(
                        "Err sending quotient of {:?} divided by {:?}: {:?}",
                        first_number, second_number, why
                    );
                }
            } else {
                error!("Err dividing {} by {}.", first_number, second_number);
                msg.channel_id.say(&ctx.http, "There was an error while dividing.");
            }
        }
        "add" => {
            info!("Got operation: {}", pre_operation);
            let first_number = args.single::<i64>()?; // Second argument
            let second_number = args.single::<i64>()?; // Third argument
            
            fn done(a: i64, b: i64) -> (bool, i64) {
                let a_is_numeric: Vec<bool> = String::from(a.to_string()).chars()
                    .map(|c|c.is_numeric())
                    .collect();

                let b_is_numeric: Vec<bool> = String::from(b.to_string()).chars()
                    .map(|c|c.is_numeric())
                    .collect();
                
                let check1 = !a_is_numeric.contains(&false);
                let check2 = !b_is_numeric.contains(&false);

                if check1 {
                    if check2 {
                        let check = true;
                        let output = a + b;
                        (check, output)
                    } else {
                        let check = false;
                        let output = 0;
                        (check, output)
                    }
                } else {
                    let check = false;
                    let output = 0;
                    (check, output)
                }
            }

            let (check, output) = done(first_number, second_number);
            if check == true {
                if let Err(why) = msg.channel_id.say(&ctx.http, &output.to_string()) {
                    error!(
                        "Err sending addition of {:?} and {:?}: {:?}",
                        first_number, second_number, why
                    );
                }
                return Ok(())
            } else if check == false {
                error!("Err adding {} and {}.", first_number, second_number);
                msg.channel_id.say(&ctx.http, "There was an error while subtracting.");
            }
        }
        "subtract" => {
            info!("Got operation: {}", pre_operation);
            let first_number = args.single::<i64>()?; // Second argument
            let second_number = args.single::<i64>()?; // Third argument
            
            if let output = first_number - second_number {
                if let Err(why) = msg.channel_id.say(&ctx.http, &output.to_string()) {
                    error!(
                        "Err sending subtraction of {:?} by {:?}: {:?}",
                        first_number, second_number, why
                    );
                }
            } else {
                error!("Err subtracting {} from {}.", second_number, first_number);
                msg.channel_id.say(&ctx.http, "There was an error while subtracting.");
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
