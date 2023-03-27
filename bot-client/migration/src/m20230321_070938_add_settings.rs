use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Guild::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(Guild::DefaultChannelId).big_integer().null(),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(Guild::ExecutionRoleId).big_integer().null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Guild::Table)
                    .drop_column(Guild::ExecutionRoleId)
                    .drop_column(Guild::DefaultChannelId)
                    .to_owned(),
            )
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Guild {
    Table,
    DefaultChannelId,
    ExecutionRoleId,
}
