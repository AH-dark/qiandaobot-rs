use rudi::Transient;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use migration::{Migrator, MigratorTrait};

use crate::libs::env::get_env;

static DATABASE_URL: &str = "DATABASE_URL";

#[Transient]
async fn init_connection() -> DatabaseConnection {
    log::debug!("Initializing the database connection");
    let options = ConnectOptions::new(get_env(DATABASE_URL));
    let db = Database::connect(options).await.unwrap_or_else(|err| {
        log::error!("Cannot connect to the database: {:?}", err);
        panic!("Cannot connect to the database: {:?}", err)
    });

    let migrations = Migrator::get_pending_migrations(&db).await.unwrap_or_else(|err| {
        log::error!("Cannot get pending migrations: {:?}", err);
        panic!("Cannot get pending migrations: {:?}", err)
    });

    if !migrations.is_empty() {
        log::info!("Applying {:?} pending migrations", migrations.len());
        Migrator::up(&db, None).await.unwrap_or_else(|err| {
            log::error!("Cannot apply pending migrations: {:?}", err);
            panic!("Cannot apply pending migrations: {:?}", err)
        });
    }

    db
}
