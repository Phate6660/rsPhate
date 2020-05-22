use std::process::Command;
use std::{collections::HashMap, sync::Arc};
use std::{env, path::Path};

use serenity::{
    client::bridge::gateway::ShardManager,
    framework::standard::{
        macros::{command, group},
        Args, CommandResult, DispatchError, StandardFramework,
    },
    http::AttachmentType,
    model::{channel::Message, gateway::Ready},
    prelude::*,
    utils::MessageBuilder,
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
    // Also set the activity to "^ls to view commands".
    fn ready(&self, ctx: Context, ready: Ready) {
        use serenity::model::gateway::Activity;
        use serenity::model::user::OnlineStatus;

        let activity = Activity::playing("^ls to view commands");
        let status = OnlineStatus::Online;

        ctx.set_presence(Some(activity), status);
        println!("{} is connected!", ready.user.name);
    }
}

#[group]
#[commands(about, ls, msg, quit)]
struct General;

#[group]
#[commands(date, rr, wipltrn, ww)]
struct Functions;

fn main() {
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

    // Commands are equivalent to:
    // "^about"
    // "^date"
    // "^ls"
    // "^msg"
    // "^rr"
    // "^wipltrn"
	// "^ww {steam,systemd}"
    // "^quit"
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
                    // You can set multiple delimiters via delimiters()
                    // or just one via delimiter(",")
                    // If you set multiple delimiters, the order you list them
                    // decides their priority (from first to last).
                    //
                    // In this case, if "," would be first, a message would never
                    // be delimited at ", ", forcing you to trim your arguments if you
                    // want to avoid whitespaces at the start of each.
                    .delimiters(vec![", ", ","])
            })
            // Set a function to be called prior to each command execution. This
            // provides the context of the command, the message that was received,
            // and the full name of the command that will be called.
            //
            // You can not use this to determine whether a command should be
            // executed. Instead, the `#[check]` macro gives you this functionality.
            .before(|ctx, msg, command_name| {
                println!(
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
                Ok(()) => println!("Processed command '{}'", command_name),
                Err(why) => println!("Command '{}' returned error {:?}", command_name, why),
            })
            // Set a function that's called whenever an attempted command-call's
            // command could not be found.
            .unrecognised_command(|_, _, unknown_command_name| {
                println!("Could not find command named '{}'", unknown_command_name);
            })
            // Set a function that's called whenever a message is not a command.
            .normal_message(|_, message| {
                println!("Message is not a command '{}'", message.content);
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
            // The `#[group]` macro generates `static` instances of the options set for the group.
            // They're made in the pattern: `#name_GROUP` for the group instance and `#name_GROUP_OPTIONS`.
            // #name is turned all uppercase
            .group(&GENERAL_GROUP)
            .group(&FUNCTIONS_GROUP),
    );

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}

#[command]
fn about(ctx: &mut Context, msg: &Message) -> CommandResult {
    let msg = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("`rsPhate`");
            e.description("A bot with random and probably nonsensical features, made by Phate6660.\nWhere to find the author:");
            // false = not inline
			e.fields(vec![
                ("Github", "https://github.com/Phate6660", false),
				("Reddit", "https://reddit.com/u/Valley6660", false),
                ("Lobsters", "https://lobste.rs/u/Phate6660", false),
				("Personal Site", "https://Phate6660.github.io/Main.html", false),
				("Discord", "@Phate6660#6270", false),
            ]);
            e
        });
        m
    });

    if let Err(why) = msg {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}

#[command]
fn date(ctx: &mut Context, msg: &Message) -> CommandResult {
    // `date` equals the output of the command `date +"%I:%M %p | %a %d, %b of %Y"`.
    let date = Command::new("date")
        .arg("+`%I:%M %p | %a %d, %b of %Y`")
        .output()
        .expect("Couldn't obtain current date and time.");
    // Sending a message can fail, due to a network error, an
    // authentication error, or lack of permissions to post in the
    // channel, so log to stdout when some error happens, with a
    // description of it.
    if let Err(why) = msg.channel_id.say(&ctx.http, String::from_utf8_lossy(&date.stdout)) {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}

#[command]
fn ls(ctx: &mut Context, msg: &Message) -> CommandResult {
    let msg = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("`^ls`");
            e.description("List available commands.");
            e.fields(vec![
				("`^about`", "Information about the author and the bot. (But mostly the author.)", false),
                ("`^date`", "Bot will reply with the date in the format -- `06:30 AM | Thu 21, May of 2020`.", false),
				("`^msg`", "Direct message user with list of commands.", false),
				("`^rr`", "Bot will reply with a link to Rick Astley's \"Never Gonna Give You Up\" without a link preview.", false),
				("`^wipltrn`", "What is Phate listening to right now?", false),
				("`^ww {steam,systemd}`", "Bot will reply with pretty embed explaining why the topic is bad.", false),
				("`^quit`", "Bot will reply with \"Shutting down now!\" and shut itself down directly after.", false),
            ]);
            e
        });
        m
    });

    if let Err(why) = msg {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}

#[command]
fn msg(ctx: &mut Context, msg: &Message) -> CommandResult {
    // Bot will send embed as DM.
    let dm = msg.author.dm(&ctx, |m| {
		m.content("Hello! Here's your personal list of commands!");
        m.embed(|e| {
            e.title("`^ls`");
            e.description("List available commands.");
            e.fields(vec![
				("`^about`", "Information about the author and the bot. (But mostly the author.)", false),
                ("`^date`", "Bot will reply with the date in the format -- `06:30 AM | Thu 21, May of 2020`.", false),
				("`^msg`", "Direct message user with list of commands.", false),
				("`^rr`", "Bot will reply with a link to Rick Astley's \"Never Gonna Give You Up\" without a link preview.", false),
				("`^wipltrn`", "What is Phate listening to right now?", false),
                ("`^ww {steam,systemd}`", "Bot will reply with pretty embed explaining why the topic is bad.", false),
				("`^quit`", "Bot will reply with \"Shutting down now!\" and shut itself down directly after.", false),
            ]);
            e
        });
        m
    });

    if let Err(why) = dm {
        println!("Error when direct messaging user: {:?}", why);
    }

    // The message builder allows for creating a message by
    // mentioning users dynamically, pushing "safe" versions of
    // content (such as bolding normalized content), displaying
    // emojis, and more.
    //
    // Bot will send "Your message has been sent in a DM, **user**."
    let response = MessageBuilder::new()
        .push("Your message has been sent in a DM, ")
        .push_bold_safe(&msg.author.name)
        .push(".")
        .build();

    if let Err(why) = msg.channel_id.say(&ctx.http, &response) {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}

#[command]
fn rr(ctx: &mut Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.channel_id.say(&ctx.http, "<https://invidio.us/watch?v=dQw4w9WgXcQ&local=1>") {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}

#[command]
fn wipltrn(ctx: &mut Context, msg: &Message) -> CommandResult {
    // `mus_title` = artist - title
    let mus_title = Command::new("sh")
        .arg("-c")
        .arg("mpc -f \"%artist% - %title%\" | head -n1")
        .output()
        .expect("Could not obtain artist and title.");
    // `mus_album` = album (date)
    let mus_album = Command::new("sh")
        .arg("-c")
        .arg("mpc -f \"%album% (%date%)\" | head -n1")
        .output()
        .expect("Could not obtain album and date.");
    let msg = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title(String::from_utf8_lossy(&mus_title.stdout));
            e.description(String::from_utf8_lossy(&mus_album.stdout));
            e.image("attachment://cover.png");
            e
        });
        m.add_file(AttachmentType::Path(Path::new("/tmp/cover.png")));
        m
    });

    if let Err(why) = msg {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}

#[command]
fn ww(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let arg = args.rest();

    if arg == "steam" {
		let msg = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Why Phate6660 hates Steam:");
                e.fields(vec![
                    ("DRM", "You don't own the games you buy, you own the right to play the game off of Steam.", false),
				    ("Mistreatment of Developers", "Steam mistreats game devs and publishers (this is why you see more and more games using GOG or even their own launchers/stores).", false),
                    ("It is Forced Onto You", "Steam is *forced* onto you by various games. Imagine my surprise when I buy the Elder Scrolls anthology (as a physical collector's set complete with DISC COPIES of the game), and every game works... except for Skyrim. Skyrim requires you to use Steam. It's a shame that I saved up 50 whole dollars for shit. I can tell you Skyrim went straight into the trash, right after being broken into 600 little pieces.", false),
				    ("Privacy Violations", "I shouldn't have to explain this one right?", false),
					("Centralization", "Having all of your games centralized into one place is stupid, and this ties into the DRM point. If Steam were to shut down right now, I guarantee that you would lose access to at least 80% of your games.", false),
                ]);
                e
            });
            m
        });

        if let Err(why) = msg {
            println!("Error sending message: {:?}", why);
        }
    }
	
    if arg == "systemd" {
		let msg = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Why Phate6660 hates SystemD:");
                e.fields(vec![
                    ("Feature Creep", "The undeniable feature creep. While some people actually enjoy the features brought in from it (boot manager, network manager, login manager?, etc), I find them to be nothing but bloat. An init system should be just that. An init system. Not <insert an exaggerated amount of functions here>.", false),
				    ("Slow", "It is slow. Slow to shutdown, slow to boot up, etc. Here are actual timed reboots from my machine using 3 init systems. SystemD (17s), OpenRC (11s), Runit (7s). 17s vs 7s, which would you choose?", false),
                    ("Bugs and Insecurities", "Due to the feature creep, there is a larger attack service for bugs and security vulnerabilities. And there are security issues with SystemD.", false),
				    ("Devs don't Care", "This is the one that bothers me the most. It's almost as if the dev(s) are completely oblivious or at least ignorant to the feature creep and security issues. Hell, Poettering even got awarded by Red Hat for making lame excuses for not fixing important bugs.", false),
                ]);
                e
            });
            m
        });

        if let Err(why) = msg {
            println!("Error sending message: {:?}", why);
        }
    }

    Ok(())
}


#[command]
fn quit(ctx: &mut Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.channel_id.say(&ctx.http, "Shutting down now!") {
        println!("Error sending message: {:?}", why);
    }

    std::process::exit(1); // Exit code of 1 to let myself know the bot was shutdown via command.
}
