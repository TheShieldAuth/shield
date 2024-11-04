//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use super::sea_orm_active_enums::GroupAccess;
use super::sea_orm_active_enums::GroupRole;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "group")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub role: GroupRole,
    pub access: GroupAccess,
    pub locked_at: Option<DateTimeWithTimeZone>,
    pub client_id: Uuid,
    pub realm_id: Uuid,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::realm::Entity",
        from = "Column::RealmId",
        to = "super::realm::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Realm,
    #[sea_orm(has_many = "super::user_group::Entity")]
    UserGroup,
}

impl Related<super::realm::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Realm.def()
    }
}

impl Related<super::user_group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserGroup.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_group::Relation::User.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::user_group::Relation::Group.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
