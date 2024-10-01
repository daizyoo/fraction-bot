use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn hello(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "hello!").await?;

    Ok(())
}
