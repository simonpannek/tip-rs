use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create Guild
        manager
            .create_table(
                Table::create()
                    .table(Guild::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Guild::Id).big_integer().primary_key())
                    .col(
                        ColumnDef::new(Guild::Ignore)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await?;

        // Create User
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).big_integer().primary_key())
                    .to_owned(),
            )
            .await?;

        // Create Event
        manager
            .create_table(
                Table::create()
                    .table(Event::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Event::Id)
                            .integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Event::Name).text().not_null())
                    .col(ColumnDef::new(Event::GuildId).big_integer().not_null())
                    .col(ColumnDef::new(Event::OwnerId).big_integer().null())
                    .index(
                        Index::create()
                            .name("idx-event-id-name")
                            .col(Event::Id)
                            .col(Event::Name)
                            .unique(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-event-guild_id")
                            .from(Event::Table, Event::GuildId)
                            .to(Guild::Table, Guild::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-event-owner_id")
                            .from(Event::Table, Event::OwnerId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        // Create EventMember
        manager
            .create_table(
                Table::create()
                    .table(EventMember::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(EventMember::EventId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(EventMember::UserId).big_integer().not_null())
                    .primary_key(
                        Index::create()
                            .name("pk-event_member")
                            .col(EventMember::EventId)
                            .col(EventMember::UserId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-event_member-event_id")
                            .from(EventMember::Table, EventMember::EventId)
                            .to(Event::Table, Event::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-event_member-user_id")
                            .from(EventMember::Table, EventMember::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop EventMember
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(EventMember::Table)
                    .name("fk-event_member-user_id")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(EventMember::Table)
                    .name("fk-event_member-event_id")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .table(EventMember::Table)
                    .name("pk-event_member")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(EventMember::Table).to_owned())
            .await?;

        // Drop Event
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Event::Table)
                    .name("fk-event-owner_id")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Event::Table)
                    .name("fk-event-guild_id")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .table(Event::Table)
                    .name("idx-event-id-name")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Event::Table).to_owned())
            .await?;

        // Drop User
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        // Drop Guild
        manager
            .drop_table(Table::drop().table(Guild::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Guild {
    Table,
    Id,
    Ignore,
}

#[derive(Iden)]
enum User {
    Table,
    Id,
}

#[derive(Iden)]
enum Event {
    Table,
    Id,
    Name,
    GuildId,
    OwnerId,
}

#[derive(Iden)]
enum EventMember {
    Table,
    EventId,
    UserId,
}
