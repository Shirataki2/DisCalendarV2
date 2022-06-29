#[derive(Debug, thiserror::Error)]
pub enum BotError {
    #[error("Invalid Date: {0}/{1}/{2} {3}:{4}")]
    InvalidDate(i32, u32, u32, u32, u32),
    #[error("{0}")]
    Serenity(#[from] poise::serenity::Error),
    #[error("{0}")]
    Database(#[from] sqlx::Error),
    #[error("{0}")]
    SlashError(#[from] poise::SlashArgError),
    #[error("`{1}`\n{0}")]
    UserError(String, String),
    #[error("{0}")]
    IoError(#[from] std::io::Error),
}

impl BotError {
    pub fn new(description: &str, ty: &str) -> Self {
        Self::UserError(description.to_string(), ty.to_string())
    }
}

pub async fn on_error(error: poise::FrameworkError<'_, crate::data::Data, BotError>) {
    // match ctx {
    //     poise::ErrorContext::Setup => panic!("Failed to start bot: {:?}", error),
    //     poise::ErrorContext::Command(ctx) => {
    //         match error {
    //             BotError::InvalidDate(year, month, day, hour, minute) => {
    //                 let _ = poise::send_reply(ctx.ctx(), |f| {
    //                     f.ephemeral(true);
    //                     f.content(format!(
    //                         "日付の入力が不正です: `{}/{}/{} {}:{}`",
    //                         year, month, day, hour, minute
    //                     ))
    //                 }).await;
    //             }
    //             _ => {
    //                 error!("Error in command `{}`: {:?}", ctx.command().name(), error)
    //             }
    //         }
    //     }
    //     _ => error!("Other error: {:?}", error),
    // }
    error!("{:?}", error);
}
