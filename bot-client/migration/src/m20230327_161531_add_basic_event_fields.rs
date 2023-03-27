use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Event::Table)
                    .add_column_if_not_exists(ColumnDef::new(Event::Description).text().not_null())
                    .add_column_if_not_exists(ColumnDef::new(Event::ChannelId).big_integer().null())
                    .add_column_if_not_exists(ColumnDef::new(Event::RoleId).big_integer().null())
                    .add_column_if_not_exists(
                        ColumnDef::new(Event::AnnouncementChannelId)
                            .big_integer()
                            .null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Event::Table)
                    .drop_column(Event::AnnouncementChannelId)
                    .drop_column(Event::RoleId)
                    .drop_column(Event::ChannelId)
                    .drop_column(Event::Description)
                    .to_owned(),
            )
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Event {
    Table,
    Description,
    ChannelId,
    RoleId,
    AnnouncementChannelId,
}
