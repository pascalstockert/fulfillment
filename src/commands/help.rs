use serenity::{
    model::{channel::Message},
    prelude::*,
};

const HELP_MESSAGE: &str = "
Hi, I'm Pasu's first attempt at Rust code!
";

pub const HELP_COMMAND: &str = "+help";

pub async fn help(ctx: &Context, msg: &Message) {
    if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
        println!("Error sending message: {:?}", why);
    }
}
