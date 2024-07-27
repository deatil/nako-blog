use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "nako_user"
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: u32,
    pub username: String,
    pub password: Option<String>,
    pub nickname: String,
    pub avatar: Option<String>,
    pub sign: Option<String>,
    pub status: Option<i32>,
    pub add_time: Option<i64>,
    pub add_ip: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::art::Entity")]
    Art,
}

impl Related<super::art::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Art.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
