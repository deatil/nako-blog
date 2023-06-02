use sea_orm::*;

use crate::app::entity::{
    comment, 
    comment::Entity as Comment,
};

/// 条件
#[derive(Clone)]
pub struct CommentWhere {
    pub art_id: Option<u32>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub content: Option<String>,
    pub status: Option<i32>,
}

impl CommentWhere {
    /// 格式化
    pub fn format(&self) -> Self {
        let mut art_id = None;
        if self.art_id != Some(0) {
            art_id = self.art_id;
        }
    
        let mut username = None;
        if self.username != Some("".to_string()) {
            username = self.username.clone();
        }
    
        let mut email = None;
        if self.email != Some("".to_string()) {
            email = self.email.clone();
        }
    
        let mut content = None;
        if self.content != Some("".to_string()) {
            content = self.content.clone();
        }
    
        let mut status = None;
        if self.status == Some(1) || self.status == Some(0) {
            status = self.status;
        }
    
        Self {
            art_id: art_id,
            username: username,
            email: email,
            content: content,
            status: status,
        }
    }
}

pub struct CommentModel;

impl CommentModel {
    pub async fn find_by_id(db: &DbConn, id: u32) -> Result<Option<comment::Model>, DbErr> {
        Comment::find_by_id(id).one(db).await
    }

    pub async fn find_count(db: &DbConn) -> Result<u64, DbErr> {
        Comment::find().count(db).await
    }

    pub async fn find_in_page(
        db: &DbConn,
        page: u64,
        per_page: u64,
    ) -> Result<(Vec<comment::Model>, u64), DbErr> {
        let paginator = Comment::find()
            .order_by_desc(comment::Column::Id)
            .paginate(db, per_page);
        let num_pages = paginator.num_pages().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    pub async fn find_count_by_artid(
        db: &DbConn, 
        artid: u32,
    ) -> Result<u64, DbErr> {
        Comment::find()
            .filter(comment::Column::ArtId.eq(artid))
            .filter(comment::Column::Status.eq(1))
            .count(db)
            .await
    }

    pub async fn find_in_page_by_artid(
        db: &DbConn,
        artid: u32,
        page: u64,
        per_page: u64,
    ) -> Result<(Vec<comment::Model>, u64), DbErr> {
        let paginator = Comment::find()
            .filter(comment::Column::ArtId.eq(artid))
            .filter(comment::Column::Status.eq(1))
            .order_by_desc(comment::Column::AddTime)
            .paginate(db, per_page);
        let num_pages = paginator.num_pages().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    // 搜索
    pub async fn search_count(
        db: &DbConn,
        wheres: CommentWhere,
    ) -> Result<u64, DbErr> {
        Comment::find()
            .apply_if(wheres.art_id, |query, v| {
                query.filter(comment::Column::ArtId.eq(v))
            })
            .apply_if(wheres.username, |query, v| {
                query.filter(comment::Column::Username.contains(format!("%{}%", v).as_str()))
            })
            .apply_if(wheres.email, |query, v| {
                query.filter(comment::Column::Email.contains(format!("%{}%", v).as_str()))
            })
            .apply_if(wheres.content, |query, v| {
                query.filter(comment::Column::Content.contains(format!("%{}%", v).as_str()))
            })
            .apply_if(wheres.status, |query, v| {
                query.filter(comment::Column::Status.eq(v))
            })
            .count(db)
            .await
    }

    pub async fn search_in_page(
        db: &DbConn,
        page: u64,
        per_page: u64,
        wheres: CommentWhere,
    ) -> Result<(Vec<comment::Model>, u64), DbErr> {
        let paginator = Comment::find()
            .apply_if(wheres.art_id, |query, v| {
                query.filter(comment::Column::ArtId.eq(v))
            })
            .apply_if(wheres.username, |query, v| {
                query.filter(comment::Column::Username.contains(format!("%{}%", v).as_str()))
            })
            .apply_if(wheres.email, |query, v| {
                query.filter(comment::Column::Email.contains(format!("%{}%", v).as_str()))
            })
            .apply_if(wheres.content, |query, v| {
                query.filter(comment::Column::Content.contains(format!("%{}%", v).as_str()))
            })
            .apply_if(wheres.status, |query, v| {
                query.filter(comment::Column::Status.eq(v))
            })
            .order_by_desc(comment::Column::Id)
            .paginate(db, per_page);
        let num_pages = paginator.num_pages().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    pub async fn create(
        db: &DbConn,
        form_data: comment::Model,
    ) -> Result<comment::ActiveModel, DbErr> {
        comment::ActiveModel {
                art_id:   Set(form_data.art_id.to_owned()),
                username: Set(form_data.username.to_owned()),
                email:    Set(form_data.email.to_owned()),
                content:  Set(form_data.content.to_owned()),
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
        form_data: comment::Model,
    ) -> Result<comment::Model, DbErr> {
        let comment: comment::ActiveModel = Comment::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find comment.".to_owned()))
            .map(Into::into)?;

        comment::ActiveModel {
                id: comment.id,
                username: Set(form_data.username.to_owned()),
                email:    Set(form_data.email.to_owned()),
                content:  Set(form_data.content.to_owned()),
                status:   Set(form_data.status.to_owned()),
                ..Default::default()
            }
            .update(db)
            .await
    }

    pub async fn update_status_by_id(
        db: &DbConn,
        id: u32,
        form_data: comment::Model,
    ) -> Result<comment::Model, DbErr> {
        let comment: comment::ActiveModel = Comment::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find comment.".to_owned()))
            .map(Into::into)?;

        comment::ActiveModel {
                id: comment.id,
                status: Set(form_data.status.to_owned()),
                ..Default::default()
            }
            .update(db)
            .await
    }

    pub async fn delete(db: &DbConn, id: u32) -> Result<DeleteResult, DbErr> {
        let comment: comment::ActiveModel = Comment::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find comment.".to_owned()))
            .map(Into::into)?;

        comment.delete(db).await
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Comment::delete_many().exec(db).await
    }

}
