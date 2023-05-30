use sea_orm::*;

use crate::app::entity::{
    cate, 
    cate::Entity as Cate,
};

/// 条件
#[derive(Clone)]
pub struct CateWhere {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub status: Option<i32>,
}

impl CateWhere {
    /// 格式化
    pub fn format(&self) -> Self {
        let mut name = None;
        if self.name != Some("".to_string()) {
            name = self.name.clone();
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
            name: name,
            slug: slug,
            status: status,
        }
    }
}

pub struct CateModel;

impl CateModel {
    pub async fn find_by_id(db: &DbConn, id: u32) -> Result<Option<cate::Model>, DbErr> {
        Cate::find_by_id(id).one(db).await
    }

    pub async fn find_by_slug(db: &DbConn, slug: &str) -> Result<Option<cate::Model>, DbErr> {
        Cate::find()
            .filter(cate::Column::Slug.contains(slug))
            .one(db)
            .await
    }

    pub async fn find_count(db: &DbConn) -> Result<u64, DbErr> {
        Cate::find().count(db).await
    }

    pub async fn find_in_page(
        db: &DbConn,
        page: u64,
        per_page: u64,
    ) -> Result<(Vec<cate::Model>, u64), DbErr> {
        let paginator = Cate::find()
            .order_by_asc(cate::Column::Id)
            .paginate(db, per_page);
        let num_pages = paginator.num_pages().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    // 搜索
    pub async fn search_count(
        db: &DbConn,
        wheres: CateWhere,
    ) -> Result<u64, DbErr> {
        Cate::find()
            .apply_if(wheres.name, |query, v| {
                query.filter(cate::Column::Name.contains(format!("%{}%", v).as_str()))
            })
            .apply_if(wheres.slug, |query, v| {
                query.filter(cate::Column::Slug.contains(format!("%{}%", v).as_str()))
            })
            .apply_if(wheres.status, |query, v| {
                query.filter(cate::Column::Status.eq(v))
            })
            .count(db)
            .await
    }

    pub async fn search_in_page(
        db: &DbConn,
        page: u64,
        per_page: u64,
        wheres: CateWhere,
    ) -> Result<(Vec<cate::Model>, u64), DbErr> {
        let paginator = Cate::find()
            .apply_if(wheres.name, |query, v| {
                query.filter(cate::Column::Name.contains(format!("%{}%", v).as_str()))
            })
            .apply_if(wheres.slug, |query, v| {
                query.filter(cate::Column::Slug.contains(format!("%{}%", v).as_str()))
            })
            .apply_if(wheres.status, |query, v| {
                query.filter(cate::Column::Status.eq(v))
            })
            .order_by_asc(cate::Column::Id)
            .paginate(db, per_page);
        let num_pages = paginator.num_pages().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    pub async fn find_all(
        db: &DbConn,
    ) -> Result<Vec<cate::Model>, DbErr> {
        Cate::find()
            .order_by_asc(cate::Column::Id)
            .all(db)
            .await
    }

    pub async fn create(
        db: &DbConn,
        form_data: cate::Model,
    ) -> Result<cate::ActiveModel, DbErr> {
        cate::ActiveModel {
                pid:      Set(form_data.pid.to_owned()),
                name:     Set(form_data.name.to_owned()),
                slug:     Set(form_data.slug.to_owned()),
                desc:     Set(form_data.desc.to_owned()),
                sort:     Set(form_data.sort.to_owned()),
                list_tpl: Set(form_data.list_tpl.to_owned()),
                view_tpl: Set(form_data.view_tpl.to_owned()),
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
        form_data: cate::Model,
    ) -> Result<cate::Model, DbErr> {
        let cate: cate::ActiveModel = Cate::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find cate.".to_owned()))
            .map(Into::into)?;

        cate::ActiveModel {
                id:       cate.id,
                pid:      Set(form_data.pid.to_owned()),
                name:     Set(form_data.name.to_owned()),
                slug:     Set(form_data.slug.to_owned()),
                desc:     Set(form_data.desc.to_owned()),
                sort:     Set(form_data.sort.to_owned()),
                list_tpl: Set(form_data.list_tpl.to_owned()),
                view_tpl: Set(form_data.view_tpl.to_owned()),
                status:   Set(form_data.status.to_owned()),
                ..Default::default()
            }
            .update(db)
            .await
    }

    pub async fn update_status_by_id(
        db: &DbConn,
        id: u32,
        form_data: cate::Model,
    ) -> Result<cate::Model, DbErr> {
        let cate: cate::ActiveModel = Cate::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find cate.".to_owned()))
            .map(Into::into)?;

        cate::ActiveModel {
                id: cate.id,
                status: Set(form_data.status.to_owned()),
                ..Default::default()
            }
            .update(db)
            .await
    }

    pub async fn delete(db: &DbConn, id: u32) -> Result<DeleteResult, DbErr> {
        let cate: cate::ActiveModel = Cate::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find cate.".to_owned()))
            .map(Into::into)?;

        cate.delete(db).await
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Cate::delete_many().exec(db).await
    }

}
