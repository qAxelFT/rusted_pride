use crate::{Context, Error};
use poise::serenity_prelude as serenity;

#[poise::command(slash_command, prefix_command)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong!").await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn sum(
    ctx: Context<'_>,
    #[description = "first number"] num1: i32,
    #[description = "Second number"] num2: i32,
) -> Result<(), Error> {
    let res = num1 + num2;

    let response = format!("{} + {} = {}", num1, num2, res);
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn echo(
    ctx: Context<'_>,
    #[description = "message to echo"] msg: String,
) -> Result<(), Error> {
    ctx.say(msg).await?;
    Ok(())
}
