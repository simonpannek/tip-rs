//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "survey_question")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub event_id: i64,
    pub message_id: i64,
    #[sea_orm(column_type = "JsonBinary")]
    pub question_data: Json,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::event::Entity",
        from = "Column::EventId",
        to = "super::event::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Event,
    #[sea_orm(has_many = "super::survey_response::Entity")]
    SurveyResponse,
}

impl Related<super::event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Event.def()
    }
}

impl Related<super::survey_response::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SurveyResponse.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
