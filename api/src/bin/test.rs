use serenity::builder::CreateInteractionResponseFollowup;

pub fn success<'a, 'b>(
    followup: &'a mut serenity::builder::CreateInteractionResponseFollowup<'b>,
    title: String,
    content: String,
    author: String,
    icon: String,
) -> &'a mut CreateInteractionResponseFollowup<'b> {
    followup.create_embed(|embed| {
        embed
            .title(title)
            .description(content)
            // .timestamp(Utc::now())
            .footer(|footer| footer.text(author).icon_url(icon))
    })
}

fn main() {
    println!("Hello, world!");
}
