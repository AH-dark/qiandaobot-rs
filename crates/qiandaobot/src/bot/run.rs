use std::sync::Arc;

use rudi::Singleton;
use teloxide::Bot;
use teloxide::repls::CommandReplExt;
use teloxide::types::Message;

use crate::bot::client::Client;
use crate::bot::command::Command;
use crate::bot::handlers::{CommandHandler, Handlers};

#[Singleton]
async fn run(client: Client, handlers: Handlers) {
    let handlers = Arc::new(handlers);
    let bot = client.bot().to_owned();

    Command::repl(bot, move |bot: Bot, msg: Message, cmd: Command| {
        let handlers = handlers.clone();

        async move {
            match cmd {
                Command::Start => handlers.start(&bot, &msg).await?,
                Command::Help => handlers.help(&bot, &msg).await?,
                Command::Qiandao => handlers.qiandao(&bot, &msg).await?,
            }

            Ok(())
        }
    }).await;
}