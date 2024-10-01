use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::fraction_calc::fraction_calculation;

#[command]
async fn fraction(ctx: &Context, msg: &Message) -> CommandResult {
    let message = &msg.content;
    let mut vec = message.split(' ').collect::<Vec<&str>>();
    vec.remove(0);

    let fraction = String::new();

    msg.channel_id
        .say(&ctx.http, fraction_calculation(&fraction).to_string())
        .await?;

    Ok(())
}
