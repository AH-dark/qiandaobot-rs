use dotenv::dotenv;
use rudi::Context;

mod bot;
mod db;
mod libs;

#[tokio::main]
async fn main() {
    dotenv().map_err(|err| log::error!("Cannot load .env file: {:?}", err)).ok();
    pretty_env_logger::init();
    log::info!("Bot starting...");

    let mut cx = Context::auto_register();
    cx.resolve_async::<()>().await;
}
