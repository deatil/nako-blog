use sea_orm::*;

use crate::app::entity::{
    attach, 
    attach::Entity as Attach,
};

pub struct AttachModel;

impl AttachModel {
    pub async fn find_by_id(db: &DbConn, id: u32) -> Result<Option<attach::Model>, DbErr> {
        Attach::find_by_id(id).one(db).await
    }

    pub async fn find_by_md5(db: &DbConn, md5: &str) -> Result<Option<attach::Model>, DbErr> {
        Attach::find()
            .filter(attach::Column::Md5.contains(md5))
            .one(db)
            .await
    }

    pub async fn find_count(db: &DbConn) -> Result<u64, DbErr> {
        Attach::find().count(db).await
    }

    /// If ok, returns (attach models, num pages).
    pub async fn find_in_page(
        db: &DbConn,
        page: u64,
        per_page: u64,
    ) -> Result<(Vec<attach::Model>, u64), DbErr> {
        // Setup paginator
        let paginator = Attach::find()
            .order_by_asc(attach::Column::Id)
            .paginate(db, per_page);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginated users
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    pub async fn create(
        db: &DbConn,
        form_data: attach::Model,
    ) -> Result<attach::ActiveModel, DbErr> {
        attach::ActiveModel {
                name:     Set(form_data.name.to_owned()),
                path:     Set(form_data.path.to_owned()),
                ext:      Set(form_data.ext.to_owned()),
                size:     Set(form_data.size.to_owned()),
                md5:      Set(form_data.md5.to_owned()),
                status:   Set(form_data.status.to_owned()),
                add_time: Set(form_data.add_time.to_owned()),
                add_ip:   Set(form_data.add_ip.to_owned()),
                ..Default::default()
            }
            .save(db)
            .await
    }

    pub async fn update_by_id(
        db: &DbConn,
        id: u32,
        form_data: attach::Model,
    ) -> Result<attach::Model, DbErr> {
        let attach: attach::ActiveModel = Attach::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find attach.".to_owned()))
            .map(Into::into)?;

        attach::ActiveModel {
                id: attach.id,
                name:   Set(form_data.name.to_owned()),
                path:   Set(form_data.path.to_owned()),
                ext:    Set(form_data.ext.to_owned()),
                size:   Set(form_data.size.to_owned()),
                md5:    Set(form_data.md5.to_owned()),
                status: Set(form_data.status.to_owned()),
                ..Default::default()
            }
            .update(db)
            .await
    }

    pub async fn update_status_by_id(
        db: &DbConn,
        id: u32,
        form_data: attach::Model,
    ) -> Result<attach::Model, DbErr> {
        let attach: attach::ActiveModel = Attach::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find attach.".to_owned()))
            .map(Into::into)?;

        attach::ActiveModel {
                id: attach.id,
                status: Set(form_data.status.to_owned()),
                ..Default::default()
            }
            .update(db)
            .await
    }

    pub async fn delete(db: &DbConn, id: u32) -> Result<DeleteResult, DbErr> {
        let attach: attach::ActiveModel = Attach::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find attach.".to_owned()))
            .map(Into::into)?;

        attach.delete(db).await
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Attach::delete_many().exec(db).await
    }

}
