//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    #[sea_orm(unique)]
    pub path: String,
    pub spoiler: Option<String>,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::post_tags::Entity")]
    PostTags,
}

impl Related<super::post_tags::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PostTags.def()
    }
}

impl Related<super::tags::Entity> for Entity {
    fn to() -> RelationDef {
        super::post_tags::Relation::Tags.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::post_tags::Relation::Posts.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
