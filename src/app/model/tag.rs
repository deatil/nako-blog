use sea_orm::*;

use crate::app::entity::{
    tag, 
    tag::Entity as Tag,
};

/// 条件
#[derive(Clone)]
pub struct TagWhere {
    pub name: Option<String>,
    pub status: Option<i32>,
}

impl TagWhere {
    /// 格式化
    pub fn format(&self) -> Self {
        let mut name = None;
        if self.name != Some("".to_string()) {
            name = self.name.clone();
        }
    
        let mut status = None;
        if self.status == Some(1) || self.status == Some(0) {
            status = self.status;
        }
    
        Self {
            name: name,
            status: status,
        }
    }
}

pub struct TagModel;

impl TagModel {
    pub async fn find_by_id(db: &DbConn, id: u32) -> Result<Option<tag::Model>, DbErr> {
        Tag::find_by_id(id).one(db).await
    }

    pub async fn find_by_name(db: &DbConn, name: &str) -> Result<Option<tag::Model>, DbErr> {
        Tag::find()
            .filter(tag::Column::Name.contains(name))
            .one(db)
            .await
    }

    pub async fn find_count(db: &DbConn) -> Result<u64, DbErr> {
        Tag::find().count(db).await
    }

    pub async fn find_in_page(
        db: &DbConn,
        page: u64,
        per_page: u64,
    ) -> Result<(Vec<tag::Model>, u64), DbErr> {
        let paginator = Tag::find()
            .order_by_asc(tag::Column::Id)
            .paginate(db, per_page);
        let num_pages = paginator.num_pages().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    // 搜索
    pub async fn search_count(
        db: &DbConn,
        wheres: TagWhere,
    ) -> Result<u64, DbErr> {
        Tag::find()
            .apply_if(wheres.name, |query, v| {
                query.filter(tag::Column::Name.contains(format!("%{}%", v).as_str()))
            })
            .apply_if(wheres.status, |query, v| {
                query.filter(tag::Column::Status.eq(v))
            })
            .count(db)
            .await
    }

    pub async fn search_in_page(
        db: &DbConn,
        page: u64,
        per_page: u64,
        wheres: TagWhere,
    ) -> Result<(Vec<tag::Model>, u64), DbErr> {
        let paginator = Tag::find()
            .apply_if(wheres.name, |query, v| {
                query.filter(tag::Column::Name.contains(format!("%{}%", v).as_str()))
            })
            .apply_if(wheres.status, |query, v| {
                query.filter(tag::Column::Status.eq(v))
            })
            .order_by_asc(tag::Column::Id)
            .paginate(db, per_page);
        let num_pages = paginator.num_pages().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    pub async fn create(
        db: &DbConn,
        form_data: tag::Model,
    ) -> Result<tag::ActiveModel, DbErr> {
        tag::ActiveModel {
                name:     Set(form_data.name.to_owned()),
                desc:     Set(form_data.desc.to_owned()),
                sort:     Set(form_data.sort.to_owned()),
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
        form_data: tag::Model,
    ) -> Result<tag::Model, DbErr> {
        let tag: tag::ActiveModel = Tag::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find tag.".to_owned()))
            .map(Into::into)?;

        tag::ActiveModel {
                id: tag.id,
                name:   Set(form_data.name.to_owned()),
                desc:   Set(form_data.desc.to_owned()),
                sort:   Set(form_data.sort.to_owned()),
                status: Set(form_data.status.to_owned()),
                ..Default::default()
            }
            .update(db)
            .await
    }

    pub async fn update_status_by_id(
        db: &DbConn,
        id: u32,
        form_data: tag::Model,
    ) -> Result<tag::Model, DbErr> {
        let tag: tag::ActiveModel = Tag::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find tag.".to_owned()))
            .map(Into::into)?;

        tag::ActiveModel {
                id: tag.id,
                status: Set(form_data.status.to_owned()),
                ..Default::default()
            }
            .update(db)
            .await
    }

    pub async fn delete(db: &DbConn, id: u32) -> Result<DeleteResult, DbErr> {
        let tag: tag::ActiveModel = Tag::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find tag.".to_owned()))
            .map(Into::into)?;

        tag.delete(db).await
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Tag::delete_many().exec(db).await
    }

}
