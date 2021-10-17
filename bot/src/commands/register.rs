use crate::prelude::*;

#[poise::command(prefix_command, hide_in_help)]
pub async fn register(ctx: Context<'_>, #[flag] global: bool) -> Result<(), BotError> {
    poise::samples::register_application_commands(ctx, global).await?;
    Ok(())
}
