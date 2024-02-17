use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "这是一个签到机器人，你可以使用以下命令")]
pub(super) enum Command {
    #[command(description = "开始使用")]
    Start,
    #[command(description = "显示帮助")]
    Help,
    #[command(description = "签到某人")]
    Qiandao,
}
