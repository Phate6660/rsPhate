use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
#[description = "Bot will reply with pretty embed explaining why the topic is bad."]
#[usage = "topic"]
#[example = "apple"]
#[example = "steam"]
#[example = "systemd"]
fn ww(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let arg = args.rest();

	if arg == "apple" {
		let msg = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Why Phate6660 hates Apple:");
                e.fields(vec![
                    ("They are Evil", "They are a despicable, evil, and disgusting company. I find them to be even worse than Google, and probably even Amazon. They've done some truly terrible things. Some examples: mass censorship, worker abuse (manipulation/brainwashing, sweatshops), repairing your own device is not allowed, they fully support DRM, they exploit developers.", false),
                    ("They Pretend", "They like to pretend that they are the good guys of tech. While companies like Google are extremely terrible for your privacy, at least they aren't pretending like they aren't. Apple likes to give people the illusion that you can pay for your privacy, which to put frankly, is not true at all. They still spy on you just as much, or even more than, Google does.", false),
                    ("They are Restrictive and Controlling", "They limit and control what you are allowed to do with your own device. Want to repair your Mac? Nope, can't do that. Want to install a different OS? Nope, they'll do as much as they can to stop you from doing that. The reason why I prefer Google more (not that I like them, this is more about choosing the lesser evil), is because you are allowed to do something about it. Don't want Android spying on you? Most of the time (depending on the phone brand) you can easily unlock your phone, install a custom recovery, and install a custom ROM like LineageOS without installing GApps. With iPhones, try as you may, you will never have that same amount of control that you can have on an Android device.", false),
                ]);
                e
            });
            m
        });

        if let Err(why) = msg {
            println!("Error sending message: {:?}", why);
        }
	}

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
