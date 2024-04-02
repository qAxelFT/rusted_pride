use crate::{Context, Error};

#[poise::command(slash_command, prefix_command, category = "Math")]
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
