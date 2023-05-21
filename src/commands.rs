use crate::{Context, Error};

/// Ping command
#[poise::command(prefix_command, slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("pong!").await?;

    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn echo(
    ctx: Context<'_>,
    #[rename = "text"]
    #[description = "Text that you want to echo"]
    s: String,
) -> Result<(), Error> {
    ctx.say(format!("You said: {}", s)).await?;

    Ok(())
}
