//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize, Serialize)]
#[sea_orm(table_name = "realm")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(column_type = "Text", unique)]
    pub name: String,
    #[sea_orm(column_type = "Text", unique)]
    pub slug: String,
    pub locked_at: Option<DateTime>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub max_concurrent_sessions: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::client::Entity")]
    Client,
    #[sea_orm(has_many = "super::resource_group::Entity")]
    ResourceGroup,
    #[sea_orm(has_many = "super::user::Entity")]
    User,
}

impl Related<super::client::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Client.def()
    }
}

impl Related<super::resource_group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ResourceGroup.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
