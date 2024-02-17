use sea_orm_migration::prelude::*;

use entity::message;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(message::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(message::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(message::Column::ChatId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(message::Column::MessageId)
                            .integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(message::Entity).to_owned())
            .await
    }
}
