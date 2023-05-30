use sea_orm::*;

use crate::app::entity::{
    setting, 
    setting::Entity as Setting,
};

pub struct SettingModel;

impl SettingModel {
    pub async fn find_by_key(db: &DbConn, key: &str) -> Result<Option<setting::Model>, DbErr> {
        Setting::find()
            .filter(setting::Column::Key.contains(key))
            .one(db)
            .await
    }

    pub async fn find_all(
        db: &DbConn,
    ) -> Result<Vec<setting::Model>, DbErr> {
        Setting::find()
            .order_by_asc(setting::Column::Id)
            .all(db)
            .await
    }

    pub async fn create(
        db: &DbConn,
        form_data: setting::Model,
    ) -> Result<setting::ActiveModel, DbErr> {
        setting::ActiveModel {
                key:   Set(form_data.key.to_owned()),
                value: Set(form_data.value.to_owned()),
                desc:  Set(form_data.desc.to_owned()),
                ..Default::default()
            }
            .save(db)
            .await
    }

    pub async fn update_by_key(
        db: &DbConn,
        key: &str,
        form_data: setting::Model,
    ) -> Result<setting::Model, DbErr> {
        let setting: setting::ActiveModel = Setting::find()
            .filter(setting::Column::Key.contains(key))
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find setting.".to_owned()))
            .map(Into::into)?;

        setting::ActiveModel {
                id:    setting.id,
                value: Set(form_data.value.to_owned()),
                ..Default::default()
            }
            .update(db)
            .await
    }

}
