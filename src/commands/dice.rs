use crate::{Context, Error};

use poise::serenity_prelude::{Color, CreateEmbed};
use rand::{Rng, SeedableRng};

#[derive(poise::ChoiceParameter)]
enum Dice {
    #[name = "D20"]
    D20,
}

impl Dice {
    fn max_value(&self) -> u8 {
        match self {
            Dice::D20 => 20,
        }
    }

    fn name(&self) -> &str {
        match self {
            Dice::D20 => "D20",
        }
    }
}

#[poise::command(slash_command)]
pub async fn dice(
    ctx: Context<'_>,
    #[description = "Type of dice"] dice: Option<Dice>,
) -> Result<(), Error> {
    let dice_type = dice.unwrap_or(Dice::D20);
    let dice_max_value = dice_type.max_value();

    let mut rng = rand::rngs::StdRng::from_rng(rand::thread_rng()).expect("Failed to setup RNG");
    let roll = rng.gen_range(1..=dice_max_value);

    let dice_name = dice_type.name();

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .title(format!("🎲 `{}`", dice_name))
                .description(format!("Hod: **`{}`**", roll))
                .color(Color::RED),
        ),
    )
    .await?;
    Ok(())
}
