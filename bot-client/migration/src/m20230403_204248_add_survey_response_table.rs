use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create SurveyQuestion
        manager
            .create_table(
                Table::create()
                    .table(SurveyQuestion::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SurveyQuestion::Id)
                            .big_integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(SurveyQuestion::EventId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SurveyQuestion::MessageId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SurveyQuestion::QuestionData)
                            .json_binary()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-survey_question-event_id")
                            .from(SurveyQuestion::Table, SurveyQuestion::EventId)
                            .to(Event::Table, Event::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create SurveyResponse
        manager
            .create_table(
                Table::create()
                    .table(SurveyResponse::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SurveyResponse::Id)
                            .big_integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(SurveyResponse::QuestionId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SurveyResponse::UserId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(SurveyResponse::Response).text().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-survey_response-question_id")
                            .from(SurveyResponse::Table, SurveyResponse::QuestionId)
                            .to(SurveyQuestion::Table, SurveyQuestion::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-survey_response-user_id")
                            .from(SurveyResponse::Table, SurveyResponse::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop SurveyResponse
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(SurveyResponse::Table)
                    .name("fk-survey_response-user_id")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(SurveyResponse::Table)
                    .name("fk-survey_response-question_id")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(SurveyResponse::Table).to_owned())
            .await?;

        // Drop SurveyQuestion
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(SurveyQuestion::Table)
                    .name("fk-survey_question-event_id")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(SurveyQuestion::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum SurveyQuestion {
    Table,
    Id,
    EventId,
    MessageId,
    QuestionData,
}

#[derive(Iden)]
enum SurveyResponse {
    Table,
    Id,
    QuestionId,
    UserId,
    Response,
}

#[derive(Iden)]
enum Event {
    Table,
    Id,
}

#[derive(Iden)]
enum User {
    Table,
    Id,
}
