#![allow(dead_code)]

use std::cmp::{max, min};

use crate::prelude::*;

#[derive(Debug)]
pub struct Paginator {
    rng: rand::prelude::StdRng,
    page: i32,
    per_page: i32,
    total: i32,
    total_pages: i32,
    template: serenity::CreateEmbed,
    list: Vec<serenity::EmbedField>,
}

impl Paginator {
    pub fn new(per_page: i32, template: serenity::CreateEmbed) -> Self {
        let page = 1;
        let list = Vec::new();

        Self {
            rng: rand::SeedableRng::from_entropy(),
            page,
            per_page,
            total: 0,
            total_pages: 0,
            template,
            list,
        }
    }

    pub fn add<T, U>(&mut self, name: T, value: U, inline: bool)
    where
        T: Into<String>,
        U: Into<String>,
    {
        let e = serenity::EmbedField::new(name, value, inline);
        self.add_element(e);
    }

    pub fn add_element(&mut self, elem: serenity::EmbedField) {
        self.list.push(elem);
        self.total += 1;
        self.total_pages = (self.total as f64 / self.per_page as f64).ceil() as i32;
    }

    pub fn insert_elements(&mut self, elems: &[serenity::EmbedField]) {
        self.list.extend_from_slice(elems);
        self.total += elems.len() as i32;
        self.total_pages = (self.total as f64 / self.per_page as f64).ceil() as i32;
    }

    fn unique_id(&mut self) -> u64 {
        self.rng.gen::<u64>()
    }

    pub async fn start(&mut self, ctx: Context<'_>) -> Result<(), BotError> {
        let prev_id = self.unique_id();
        let finish_id = self.unique_id();
        let next_id = self.unique_id();
        
        poise::send_reply(ctx, |f| {
            f.embed(|e| {
                let mut embed = self.template.clone();
                self.insert_fields(&mut embed);
                embed.footer(|f| {
                    f.text(format!(
                        "Page {}/{}",
                        self.page, self.total_pages
                    ))
                });
                *e = embed;
                e
            });
            f.components(|c| {
                c.create_action_row(|ar| {
                    ar.create_button(|b| {
                        b.style(serenity::ButtonStyle::Primary)
                            .label("前へ")
                            .custom_id(&prev_id)
                    });
                    ar.create_button(|b| {
                        b.style(serenity::ButtonStyle::Danger)
                            .label("完了")
                            .custom_id(&finish_id)
                    });
                    ar.create_button(|b| {
                        b.style(serenity::ButtonStyle::Primary)
                            .label("次へ")
                            .custom_id(&next_id)
                    })
                })
            })
        }).await?;

        while let Some(mci) = serenity::CollectComponentInteraction::new(ctx.discord())
            .author_id(ctx.author().id)
            .channel_id(ctx.channel_id())
            .timeout(std::time::Duration::from_secs(10 * 60))
            .filter(move |mci| {
                let id = &mci.data.custom_id;
                id == &prev_id.to_string() || id == &next_id.to_string() || id == &finish_id.to_string()
            })
            .await
        {
            let id = mci.data.custom_id.clone();
            if id == prev_id.to_string() {
                self.page = max(1, self.page - 1);
            } else if id == next_id.to_string() {
                self.page = min(self.total_pages, self.page + 1);
            }
            
            let mut msg = mci.message.clone();
            if id == finish_id.to_string() {
                msg.delete(ctx.discord()).await?;
                break;
            }

            msg.edit(ctx.discord(), |f| {
                f.embed(|e| {
                    let mut embed = self.template.clone();
                    self.insert_fields(&mut embed);
                    embed.footer(|f| {
                        f.text(format!(
                            "Page {}/{}",
                            self.page, self.total_pages
                        ))
                    });
                    *e = embed;
                    e
                });
                f.components(|c| {
                    c.create_action_row(|ar| {
                        ar.create_button(|b| {
                            b.style(serenity::ButtonStyle::Primary)
                                .label("前へ")
                                .custom_id(&prev_id)
                        });
                        ar.create_button(|b| {
                            b.style(serenity::ButtonStyle::Danger)
                                .label("完了")
                                .custom_id(&finish_id)
                        });
                        ar.create_button(|b| {
                            b.style(serenity::ButtonStyle::Primary)
                                .label("次へ")
                                .custom_id(&next_id)
                        })
                    })
                })
            }).await?;

            mci.create_interaction_response(ctx.discord(), |ir| {
                ir.kind(serenity::InteractionResponseType::DeferredUpdateMessage)
            })
            .await?;
        }

        Ok(())
    }

    fn insert_fields(&self, e: &mut serenity::CreateEmbed) {
        for i in 0..self.per_page {
            let index = i + (self.page - 1) * self.per_page;
            if index >= self.total {
                break;
            }
            let field = self.list[index as usize].clone();
            e.field(field.name, field.value, field.inline);
        }
    }
}
