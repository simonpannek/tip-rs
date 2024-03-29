//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub avatar: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::event::Entity")]
    Event,
}

impl Related<super::event::Entity> for Entity {
    fn to() -> RelationDef {
        super::event_member::Relation::Event.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::event_member::Relation::User.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
