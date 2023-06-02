use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "nako_art")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: u32,
    pub uuid: String,
    pub cate_id: u32,
    pub user_id: u32,
    pub title: String,
    pub keywords: Option<String>,
    pub description: Option<String>,
    pub cover: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub content: String,
    pub brief: Option<String>,
    pub tags: Option<String>,
    pub from: Option<String>,
    pub views: Option<u64>,
    pub is_top: Option<i32>,
    pub status: Option<i32>,
    pub add_time: Option<i64>,
    pub add_ip: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::cate::Entity",
        from = "Column::CateId",
        to = "super::cate::Column::Id"
    )]
    Cate,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::cate::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Cate.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
