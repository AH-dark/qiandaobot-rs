use rudi::Transient;
use teloxide::Bot;

#[derive(Clone)]
pub struct Client {
    bot: Bot,
}

#[Transient]
impl Client {
    #[di]
    fn new() -> Self {
        let bot = Bot::from_env();
        Self { bot }
    }

    pub fn bot(&self) -> &Bot {
        &self.bot
    }
}
