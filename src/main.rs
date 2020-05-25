use log::{error, info};
use serenity::{
    client::bridge::gateway::ShardManager,
    framework::standard::{
        help_commands,
        macros::{group, help},
        Args, CommandGroup, CommandResult, DispatchError, HelpOptions, StandardFramework,
    },
    model::{event::ResumedEvent, gateway::Ready, id::UserId, prelude::Message},
    prelude::*,
};
use std::env;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

// Load and use commands from src/commands/
mod commands;
use commands::{
    about::*, date::*, fortune::*, hmm::*, iv::*, math::*, projects::*, quit::*, rng::*, rr::*,
    wipltrn::*, ww::*,
};

// A container type is created for inserting into the Client's `data`, which
// allows for data to be accessible across all events and framework commands, or
// anywhere else that has a copy of the `data` Arc.
struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct CommandCounter;

impl TypeMapKey for CommandCounter {
    type Value = HashMap<String, u64>;
}

struct Handler;

impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    //
    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    fn ready(&self, ctx: Context, ready: Ready) {
        use serenity::model::gateway::Activity;
        use serenity::model::user::OnlineStatus;

        let activity = Activity::playing("^ls to view commands");
        let status = OnlineStatus::Online;

        ctx.set_presence(Some(activity), status);
        info!("Connected as {}", ready.user.name);
    }
    fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

// Groups
#[group]
#[description = "Functions for the bot that do not belong in any specific category."]
#[commands(date, hmm, iv, fortune, rr, wipltrn, ww)]
struct Functions;

#[group]
#[description = "Generalized functions for the bot."]
#[commands(about, projects, quit)]
struct General;

#[group]
#[description = "Functions that are related to number operations."]
#[commands(math, rng)]
struct Numbers;

#[help]
#[individual_command_tip = "If you want more information about a specific group or argument, just pass it as an argument."]
fn my_help(
    context: &mut Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    help_commands::with_embeds(context, msg, args, help_options, groups, owners)
}

fn main() {
    // This will load the environment variables located at `./.env`, relative to the CWD.
    kankyo::load().expect("Failed to load .env file");

    // Initialize the logger to use environment variables.
    env_logger::init();

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    {
        let mut data = client.data.write();
        data.insert::<CommandCounter>(HashMap::default());
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    client.with_framework(
        // Configures the client, allowing for options to mutate how the
        // framework functions.
        //
        // Refer to the documentation for
        // `serenity::ext::framework::Configuration` for all available
        // configurations.
        StandardFramework::new()
            .configure(|c| {
                c.with_whitespace(true)
                    .prefix("^")
                    // Delimiters are: " ", ", ", and ",".
                    .delimiters(vec![" ", ", ", ","])
            })
            // Set a function to be called prior to each command execution. This
            // provides the context of the command, the message that was received,
            // and the full name of the command that will be called.
            //
            // You can not use this to determine whether a command should be
            // executed. Instead, the `#[check]` macro gives you this functionality.
            .before(|ctx, msg, command_name| {
                info!(
                    "Got command '{}' by user '{}'",
                    command_name, msg.author.name
                );

                // Increment the number of times this command has been run once. If
                // the command's name does not exist in the counter, add a default
                // value of 0.
                let mut data = ctx.data.write();
                let counter = data
                    .get_mut::<CommandCounter>()
                    .expect("Expected CommandCounter in ShareMap.");
                let entry = counter.entry(command_name.to_string()).or_insert(0);
                *entry += 1;

                true // if `before` returns false, command processing doesn't happen.
            })
            // Similar to `before`, except will be called directly _after_
            // command execution.
            .after(|_, _, command_name, error| match error {
                Ok(()) => info!("Processed command '{}'", command_name),
                Err(why) => info!("Command '{}' returned error {:?}", command_name, why),
            })
            // Set a function that's called whenever an attempted command-call's
            // command could not be found.
            .unrecognised_command(|_, _, unknown_command_name| {
                error!("Could not find command named '{}'", unknown_command_name);
            })
            // Set a function that's called whenever a command's execution didn't complete for one
            // reason or another. For example, when a user has exceeded a rate-limit or a command
            // can only be performed by the bot owner.
            .on_dispatch_error(|ctx, msg, error| {
                if let DispatchError::Ratelimited(seconds) = error {
                    let _ = msg.channel_id.say(
                        &ctx.http,
                        &format!("Try this again in {} seconds.", seconds),
                    );
                }
            })
            // Set the help function
            .help(&MY_HELP)
            // The `#[group]` macro generates `static` instances of the options set for the group.
            // They're made in the pattern: `#name_GROUP` for the group instance and `#name_GROUP_OPTIONS`.
            // #name is turned all uppercase
            .group(&FUNCTIONS_GROUP)
            .group(&GENERAL_GROUP)
            .group(&NUMBERS_GROUP),
    );

    if let Err(why) = client.start() {
        error!("Client error: {:?}", why);
    }
}
