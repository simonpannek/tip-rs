use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create ScheduledAction
        manager
            .create_table(
                Table::create()
                    .table(ScheduledAction::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ScheduledAction::Id)
                            .big_integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ScheduledAction::EventId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ScheduledAction::Time)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ScheduledAction::Executed)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(ScheduledAction::ActionType)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ScheduledAction::ActionData)
                            .json_binary()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-scheduled_action-event_id")
                            .from(ScheduledAction::Table, ScheduledAction::EventId)
                            .to(Event::Table, Event::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop ScheduledAction
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(ScheduledAction::Table)
                    .name("fk-scheduled_action-event_id")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(ScheduledAction::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum ScheduledAction {
    Table,
    Id,
    EventId,
    Time,
    Executed,
    ActionType,
    ActionData,
}

#[derive(Iden)]
enum Event {
    Table,
    Id,
}
