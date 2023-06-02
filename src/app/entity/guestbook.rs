use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "nako_guestbook")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: u32,
    pub name: String,
    pub message: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub qq: Option<String>,
    pub weixin: Option<String>,
    pub status: Option<i32>,
    pub add_time: Option<i64>,
    pub add_ip: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
