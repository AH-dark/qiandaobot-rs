use log::error;
use rudi::Singleton;
use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;

use crate::bot::client::Client;
use crate::bot::command::Command;
use crate::bot::handlers::{CommandHandler, Handlers};
use crate::libs::env::get_env;

#[Singleton]
async fn run(client: Client, handlers: Handlers) {
    let bot = client.bot().clone();

    let handler = Update::filter_message()
        .branch(dptree::entry().filter_command::<Command>().endpoint(|bot: Bot, msg: Message, handlers: Handlers| async move {
            let me = bot.get_me().await?;

            let text = msg.text();
            if text.is_none() {
                return respond(());
            }

            match Command::parse(text.unwrap(), me.username()) {
                Ok(cmd) => {
                    match cmd {
                        Command::Start => handlers.start(&bot, &msg).await?,
                        Command::Help => handlers.help(&bot, &msg).await?,
                        Command::Qiandao => handlers.qiandao(&bot, &msg).await?,
                    }
                }
                Err(err) => {
                    error!("Cannot parse command: {:?}", err);
                }
            }

            respond(())
        }));

    let channel_handler = Update::filter_channel_post()
        .filter(|update: Message| update.chat.id.0 == get_env("CHANNEL_ID").parse::<i64>().unwrap_or(0))
        .endpoint(|bot: Bot, msg: Message, handlers: Handlers| async move {
            let chat_id = msg.chat.id;
            let message_id = msg.id;

            handlers.add_message(chat_id.0, message_id.0).await?;

            respond(())
        });

    // TODO: combine handler and channel_handler

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![handlers])
        .build()
        .dispatch()
        .await
}