use sea_orm::*;

use crate::app::entity::{
    art, 
    art::Entity as Art,
};

/// 条件
#[derive(Clone)]
pub struct ArtWhere {
    pub uuid: Option<String>,
    pub cate_id: Option<u32>,
    pub user_id: Option<u32>,
    pub title: Option<String>,
    pub is_top: Option<i32>,
    pub status: Option<i32>,
}

impl ArtWhere {
    /// 格式化
    pub fn format(&self) -> Self {
        let mut title = None;
        if self.title != Some("".to_string()) {
            title = self.title.clone();
        }
    
        let mut uuid = None;
        if self.uuid != Some("".to_string()) {
            uuid = self.uuid.clone();
        }
    
        let mut is_top = None;
        if self.is_top == Some(1) || self.is_top == Some(0) {
            is_top = self.is_top;
        }
    
        let mut status = None;
        if self.status == Some(1) || self.status == Some(0) {
            status = self.status;
        }
    
        Self {
            title: title,
            uuid: uuid,
            cate_id: None,
            user_id: None,
            is_top: is_top,
            status: status,
        }
    }
}

pub struct ArtModel;

impl ArtModel {
    pub async fn find_by_id(db: &DbConn, id: u32) -> Result<Option<art::Model>, DbErr> {
        Art::find_by_id(id).one(db).await
    }

    pub async fn find_by_uuid(db: &DbConn, uuid: &str) -> Result<Option<art::Model>, DbErr> {
        Art::find()
            .filter(art::Column::Uuid.contains(uuid))
            .one(db)
            .await
    }

    pub async fn find_count(db: &DbConn) -> Result<u64, DbErr> {
        Art::find().count(db).await
    }

    pub async fn find_in_page(
        db: &DbConn,
        page: u64,
        per_page: u64,
    ) -> Result<(Vec<art::Model>, u64), DbErr> {
        let paginator = Art::find()
            .order_by_asc(art::Column::Id)
            .paginate(db, per_page);
        let num_pages = paginator.num_pages().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    // 搜索
    pub async fn search_count(
        db: &DbConn,
        wheres: ArtWhere,
    ) -> Result<u64, DbErr> {
        Art::find()
            .apply_if(wheres.uuid, |query, v| {
                query.filter(art::Column::Uuid.eq(v))
            })
            .apply_if(wheres.cate_id, |query, v| {
                query.filter(art::Column::CateId.eq(v))
            })
            .apply_if(wheres.user_id, |query, v| {
                query.filter(art::Column::UserId.eq(v))
            })
            .apply_if(wheres.title, |query, v| {
                query.filter(art::Column::Title.contains(format!("%{}%", v).as_str()))
            })
            .apply_if(wheres.is_top, |query, v| {
                query.filter(art::Column::IsTop.eq(v))
            })
            .apply_if(wheres.status, |query, v| {
                query.filter(art::Column::Status.eq(v))
            })
            .count(db)
            .await
    }

    pub async fn search_in_page(
        db: &DbConn,
        page: u64,
        per_page: u64,
        wheres: ArtWhere,
    ) -> Result<(Vec<art::Model>, u64), DbErr> {
        let paginator = Art::find()
            .apply_if(wheres.uuid, |query, v| {
                query.filter(art::Column::Uuid.eq(v))
            })
            .apply_if(wheres.cate_id, |query, v| {
                query.filter(art::Column::CateId.eq(v))
            })
            .apply_if(wheres.user_id, |query, v| {
                query.filter(art::Column::UserId.eq(v))
            })
            .apply_if(wheres.title, |query, v| {
                query.filter(art::Column::Title.contains(format!("%{}%", v).as_str()))
            })
            .apply_if(wheres.is_top, |query, v| {
                query.filter(art::Column::IsTop.eq(v))
            })
            .apply_if(wheres.status, |query, v| {
                query.filter(art::Column::Status.eq(v))
            })
            .order_by_asc(art::Column::Id)
            .paginate(db, per_page);
        let num_pages = paginator.num_pages().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    pub async fn create(
        db: &DbConn,
        form_data: art::Model,
    ) -> Result<art::ActiveModel, DbErr> {
        art::ActiveModel {
                uuid:        Set(form_data.uuid.to_owned()),
                cate_id:     Set(form_data.cate_id.to_owned()),
                user_id:     Set(form_data.user_id.to_owned()),
                cover:       Set(form_data.cover.to_owned()),
                title:       Set(form_data.title.to_owned()),
                keywords:    Set(form_data.keywords.to_owned()),
                description: Set(form_data.description.to_owned()),
                content:     Set(form_data.content.to_owned()),
                tags:        Set(form_data.tags.to_owned()),
                from:        Set(form_data.from.to_owned()),
                views:       Set(form_data.views.to_owned()),
                is_top:      Set(form_data.is_top.to_owned()),
                status:      Set(form_data.status.to_owned()),
                add_time:    Set(form_data.add_time.to_owned()),
                add_ip:      Set(form_data.add_ip.to_owned()),
                ..Default::default()
            }
            .save(db)
            .await
    }

    pub async fn update_by_id(
        db: &DbConn,
        id: u32,
        form_data: art::Model,
    ) -> Result<art::Model, DbErr> {
        let art: art::ActiveModel = Art::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find art.".to_owned()))
            .map(Into::into)?;

        art::ActiveModel {
                id:          art.id,
                cate_id:     Set(form_data.cate_id.to_owned()),
                title:       Set(form_data.title.to_owned()),
                keywords:    Set(form_data.keywords.to_owned()),
                description: Set(form_data.description.to_owned()),
                cover:       Set(form_data.cover.to_owned()),
                content:     Set(form_data.content.to_owned()),
                tags:        Set(form_data.tags.to_owned()),
                from:        Set(form_data.from.to_owned()),
                is_top:      Set(form_data.is_top.to_owned()),
                status:      Set(form_data.status.to_owned()),
                add_time:    Set(form_data.add_time.to_owned()),
                ..Default::default()
            }
            .update(db)
            .await
    }

    pub async fn update_status_by_id(
        db: &DbConn,
        id: u32,
        form_data: art::Model,
    ) -> Result<art::Model, DbErr> {
        let art: art::ActiveModel = Art::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find art.".to_owned()))
            .map(Into::into)?;

        art::ActiveModel {
                id: art.id,
                status: Set(form_data.status.to_owned()),
                ..Default::default()
            }
            .update(db)
            .await
    }

    pub async fn delete(db: &DbConn, id: u32) -> Result<DeleteResult, DbErr> {
        let art: art::ActiveModel = Art::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find art.".to_owned()))
            .map(Into::into)?;

        art.delete(db).await
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Art::delete_many().exec(db).await
    }

}
