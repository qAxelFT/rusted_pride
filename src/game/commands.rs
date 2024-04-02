use crate::{Context, Error};
use rand::Rng;

#[poise::command(slash_command, prefix_command, category = "Game")]
pub async fn roll_dice(
    ctx: Context<'_>,
    #[description = "Quantity of dice to roll"] n_dice: u32,
    #[description = "Number of faces of the dice"] n_faces: u32,
) -> Result<(), Error> {
    let roll: u32;
    let dice1: u32;
    let dice2: u32;
    let _ = match n_dice {
        1 => {
            dice1 = rand::thread_rng().gen_range(1..=n_faces);
            roll = dice1;

            ctx.say(format!("Your roll was {}", roll)).await?;

            Ok::<(), Error>(())
        }
        2 => {
            dice1 = rand::thread_rng().gen_range(1..=n_faces);
            dice2 = rand::thread_rng().gen_range(1..=n_faces);

            roll = dice1 + dice2;

            ctx.say(format!(
                "Your roll was {}, first dice {}, second dice {}",
                roll, dice1, dice2
            ))
            .await?;

            Ok(())
        }

        _ => {
            ctx.say("You can use 1-2 dice only! Try again.").await?;
            Ok(())
        }
    };

    Ok(())
}
