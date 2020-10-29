use log::{error, info};
use serenity::{model::channel::{Message, ReactionType}, prelude::*};

// If the user's message is starred, copy it and send it as an embed in #starboard.
pub fn star(ctx: &mut Context, msg: &Message) {
    let sent_message = &msg.content;
    let reactions = &msg.reactions;
    info!("Content: {}, Reactions: {:#?}", sent_message, reactions);
}
