use log::{error, info};
use serenity::{
    model::channel::Message,
    prelude::*,
};

// If "youtube.com" is found, react with disappointed and message a pretty embed about invidio.
// Do the same for "twitter.com", except the embed pertains to Nitter.
pub fn yt_tw_check(ctx: &mut Context, msg: &Message) {
    let sent_message = &msg.content;
    if sent_message.contains("youtube.com") {
        info!("YouTube link was found!");
        msg.react(&ctx.http, '😞');
        let message = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("`rsPhate`");
                e.description("YouTube link discovered! Please link to invidio instead. Why:");
                // false = not inline
                e.fields(vec![
                    ("FOSS", "Invidio is a [FOSS](https://github.com/omarroth/invidious) frontend to YouTube.", false),
                    ("Proxy", "Invidio has the ability to proxy YouTube videos through their servers, allowing you to watch videos without ever touching Google's servers. Add `&local=1` to an invidio URL to proxy.", false),
                    ("Lightweight", "The entire site is very small, and works without JavaScript or XHR.", false),
                    ("Self-Hostable", "You can self-host your own instance if you really wanted to. And if you don't trust the [main instance](https://invidio.us) there are a [variety of other instances](https://github.com/omarroth/invidious/wiki/Invidious-Instances) to choose from.", false),
                    ("I'm interested, how do I convert URLs?", "Replace `youtube.com` with `invidio.us`. For `youtu.be` URLs, take the string after the last slash and append it to `https://invidio.us/watch?v=`.", false),
                ]);
                e
            });
            m
        });

        if let Err(why) = message {
            error!("Error sending message: {:?}", why);
        }
    }
    if sent_message.contains("twitter.com") {
        info!("Twitter link was found!");
        msg.react(&ctx.http, '😞');
        let message = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("`rsPhate`");
                e.description("Twitter link discovered! Please link to Nitter instead. Why:");
                // false = not inline
                e.fields(vec![
                    ("FOSS", "Nitter is a [FOSS](https://github.com/zedeus/nitter) frontend to Twitter inspired by invidio.", false),
                    ("Proxy", "Everything is proxied through Nitter, Twitter won't even know your IP, much less be able to fingerprint or track you.", false),
                    ("API", "Contains an unoffical twitter api without any keys required or any rate limits enforced.", false),
                    ("Lightweight", "Pages are very small, and work without JS and XHR.", false),
                    ("Self-Hostable", "As with invidio, it is self-hostable and there are plenty of other [instances](https://github.com/zedeus/nitter/wiki/Instances) to choose from.", false),
                    ("I'm interested, how do I convert URLs?", "Replace `twitter.com` with `nitter.net`.", false),
                ]);
                e
            });
            m
        });

        if let Err(why) = message {
            error!("Error sending message: {:?}", why);
        }
    }
}
