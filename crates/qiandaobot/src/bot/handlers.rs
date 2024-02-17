use rudi::Singleton;
use sea_orm::{DatabaseConnection, entity::*, EntityTrait, PaginatorTrait, query::*};
use teloxide::Bot;
use teloxide::prelude::*;
use teloxide::types::Message;
use teloxide::utils::command::BotCommands;
use entity::message;

use crate::bot::command::Command;

#[derive(Debug, Clone)]
#[Singleton(async)]
pub(super) struct Handlers {
    db: DatabaseConnection,
}

impl Handlers {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

pub trait CommandHandler: Sync + Send {
    async fn start(&self, bot: &Bot, msg: &Message) -> ResponseResult<()>;
    async fn help(&self, bot: &Bot, msg: &Message) -> ResponseResult<()>;
    async fn qiandao(&self, bot: &Bot, msg: &Message) -> ResponseResult<()>;
}

impl CommandHandler for Handlers {
    async fn start(&self, bot: &Bot, msg: &Message) -> ResponseResult<()> {
        bot.send_message(msg.chat.id, "这是一个签到机器人，你可以拿它来签到别人。")
            .await
            .expect("Cannot send message");

        Ok(())
    }

    async fn help(&self, bot: &Bot, msg: &Message) -> ResponseResult<()> {
        bot.send_message(msg.chat.id, Command::descriptions().to_string())
            .await
            .expect("Cannot send message");

        Ok(())
    }

    async fn qiandao(&self, bot: &Bot, msg: &Message) -> ResponseResult<()> {
        let count = message::Entity::find()
            .count(&self.db)
            .await
            .expect("Cannot count messages");
        log::debug!("Messages count: {}", count);

        Ok(())
    }
}

