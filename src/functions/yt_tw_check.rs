use log::{error, info};
use serenity::{model::channel::Message, prelude::*};

// If "youtube.com" is found, react with disappointed and message a pretty embed about invidio.
// Do the same for "twitter.com", except the embed pertains to Nitter.
pub fn tw_check(ctx: &mut Context, msg: &Message) {
    let sent_message = &msg.content;
    if sent_message.contains("twitter.com") {
        info!("Twitter link was found! It was sent by: {}", msg.author.name);
        msg.react(&ctx.http, '😞').expect("could not react");
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
