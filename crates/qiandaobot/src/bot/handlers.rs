use rand::Rng;
use rudi::Singleton;
use sea_orm::{DatabaseConnection, entity::*, EntityTrait, PaginatorTrait, query::*};
use teloxide::Bot;
use teloxide::prelude::*;
use teloxide::types::{Message, MessageId};
use teloxide::utils::command::BotCommands;

use entity::message;

use crate::bot::command::Command;

#[derive(Debug, Clone)]
#[Singleton(async)]
pub(super) struct Handlers {
    db: DatabaseConnection,
}

pub trait CommandHandler: Sync + Send {
    async fn start(&self, bot: &Bot, msg: &Message) -> ResponseResult<()>;
    async fn help(&self, bot: &Bot, msg: &Message) -> ResponseResult<()>;
    async fn qiandao(&self, bot: &Bot, msg: &Message) -> ResponseResult<()>;

    async fn add_message(&self, chat_id: i64, message_id: i32) -> ResponseResult<()>;
    async fn remove_message(&self, chat_id: i64, message_id: i32) -> ResponseResult<()>;
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

        if count == 0 {
            bot.send_message(msg.chat.id, "没有信息供签到")
                .await
                .expect("Cannot send message");
            return Ok(());
        }

        let offset = rand::thread_rng().gen_range(0..count);
        let message = message::Entity::find()
            .offset(offset)
            .one(&self.db)
            .await
            .expect("Failed to get a message");

        match message {
            Some(message) => {
                bot.forward_message(msg.chat.id, ChatId(message.chat_id), MessageId(message.message_id)).await?;
            }
            None => {
                bot.send_message(msg.chat.id, "没有信息供签到").await?;
            }
        }

        Ok(())
    }

    async fn add_message(&self, chat_id: i64, message_id: i32) -> ResponseResult<()> {
        match (
            message::ActiveModel {
                id: NotSet,
                chat_id: ActiveValue::Set(chat_id),
                message_id: ActiveValue::Set(message_id),
            }.insert(&self.db).await
        ) {
            Ok(_) => {
                log::info!("Message added: chat_id={}, message_id={}", chat_id, message_id)
            }
            Err(err) => {
                log::error!("Cannot add message: {:?}", err);
            }
        };

        Ok(())
    }

    async fn remove_message(&self, chat_id: i64, message_id: i32) -> ResponseResult<()> {
        match message::Entity::delete(
            message::ActiveModel {
                id: NotSet,
                chat_id: ActiveValue::Set(chat_id),
                message_id: ActiveValue::Set(message_id),
            }
        )
            .exec(&self.db)
            .await
        {
            Ok(_) => {}
            Err(err) => {
                log::error!("Cannot remove message: {:?}", err);
            }
        }

        Ok(())
    }
}

