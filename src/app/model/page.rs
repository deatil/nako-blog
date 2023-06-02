use sea_orm::*;

use crate::app::entity::{
    page, 
    page::Entity as Page,
};

/// 条件
#[derive(Clone)]
pub struct PageWhere {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub status: Option<i32>,
}

impl PageWhere {
    /// 格式化
    pub fn format(&self) -> Self {
        let mut title = None;
        if self.title != Some("".to_string()) {
            title = self.title.clone();
        }
    
        let mut slug = None;
        if self.slug != Some("".to_string()) {
            slug = self.slug.clone();
        }
    
        let mut status = None;
        if self.status == Some(1) || self.status == Some(0) {
            status = self.status;
        }
    
        Self {
            title: title,
            slug: slug,
            status: status,
        }
    }
}

pub struct PageModel;

impl PageModel {
    pub async fn find_by_id(db: &DbConn, id: u32) -> Result<Option<page::Model>, DbErr> {
        Page::find_by_id(id).one(db).await
    }

    pub async fn find_by_slug(db: &DbConn, slug: &str) -> Result<Option<page::Model>, DbErr> {
        Page::find()
            .filter(page::Column::Slug.eq(slug))
            .one(db)
            .await
    }

    pub async fn find_count(db: &DbConn) -> Result<u64, DbErr> {
        Page::find().count(db).await
    }

    pub async fn find_in_page(
        db: &DbConn,
        page: u64,
        per_page: u64,
    ) -> Result<(Vec<page::Model>, u64), DbErr> {
        let paginator = Page::find()
            .order_by_asc(page::Column::Id)
            .paginate(db, per_page);
        let num_pages = paginator.num_pages().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    // 搜索
    pub async fn search_count(
        db: &DbConn,
        wheres: PageWhere,
    ) -> Result<u64, DbErr> {
        Page::find()
            .apply_if(wheres.title, |query, v| {
                query.filter(page::Column::Title.contains(format!("%{}%", v).as_str()))
            })
            .apply_if(wheres.slug, |query, v| {
                query.filter(page::Column::Slug.contains(format!("%{}%", v).as_str()))
            })
            .apply_if(wheres.status, |query, v| {
                query.filter(page::Column::Status.eq(v))
            })
            .count(db)
            .await
    }

    pub async fn search_in_page(
        db: &DbConn,
        page: u64,
        per_page: u64,
        wheres: PageWhere,
    ) -> Result<(Vec<page::Model>, u64), DbErr> {
        let paginator = Page::find()
            .apply_if(wheres.title, |query, v| {
                query.filter(page::Column::Title.contains(format!("%{}%", v).as_str()))
            })
            .apply_if(wheres.slug, |query, v| {
                query.filter(page::Column::Slug.contains(format!("%{}%", v).as_str()))
            })
            .apply_if(wheres.status, |query, v| {
                query.filter(page::Column::Status.eq(v))
            })
            .order_by_asc(page::Column::Id)
            .paginate(db, per_page);
        let num_pages = paginator.num_pages().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    pub async fn create(
        db: &DbConn,
        form_data: page::Model,
    ) -> Result<page::ActiveModel, DbErr> {
        page::ActiveModel {
                user_id:     Set(form_data.user_id.to_owned()),
                slug:        Set(form_data.slug.to_owned()),
                title:       Set(form_data.title.to_owned()),
                keywords:    Set(form_data.keywords.to_owned()),
                description: Set(form_data.description.to_owned()),
                content:     Set(form_data.content.to_owned()),
                tpl:         Set(form_data.tpl.to_owned()),
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
        form_data: page::Model,
    ) -> Result<page::Model, DbErr> {
        let page: page::ActiveModel = Page::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find page.".to_owned()))
            .map(Into::into)?;

        page::ActiveModel {
                id:          page.id,
                slug:        Set(form_data.slug.to_owned()),
                title:       Set(form_data.title.to_owned()),
                keywords:    Set(form_data.keywords.to_owned()),
                description: Set(form_data.description.to_owned()),
                content:     Set(form_data.content.to_owned()),
                tpl:         Set(form_data.tpl.to_owned()),
                status:      Set(form_data.status.to_owned()),
                ..Default::default()
            }
            .update(db)
            .await
    }

    pub async fn update_status_by_id(
        db: &DbConn,
        id: u32,
        form_data: page::Model,
    ) -> Result<page::Model, DbErr> {
        let page: page::ActiveModel = Page::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find page.".to_owned()))
            .map(Into::into)?;

        page::ActiveModel {
                id: page.id,
                status: Set(form_data.status.to_owned()),
                ..Default::default()
            }
            .update(db)
            .await
    }

    pub async fn delete(db: &DbConn, id: u32) -> Result<DeleteResult, DbErr> {
        let page: page::ActiveModel = Page::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find page.".to_owned()))
            .map(Into::into)?;

        page.delete(db).await
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Page::delete_many().exec(db).await
    }

}
